use crate::{
    activity::{ActivityDetails, ActivityType, LabAudience, PTDay, ParsedOpord},
    parser_error::OpordParserError,
};

pub struct OpordParser<'a> {
    opord: &'a str,
}

#[derive(PartialEq, Debug)]
enum State {
    ReadMissionName,
    ScanUOD,
    ReadUOD,
    ScanLocation,
    ReadLocation,
    ActivityParseComplete,
    ScanMission,
}

impl<'a> OpordParser<'a> {
    pub fn new(opord: &'a str) -> Self {
        Self { opord }
    }

    pub fn parse(&self) -> Result<ParsedOpord<'a>, OpordParserError> {
        let lines = self.opord.lines();

        let mut state = State::ScanMission;

        // activity details
        let mut name = "";
        let mut uod = "";
        let mut location = "";

        let mut week_num = None;

        let mut result = vec![];

        for line in lines {
            if week_num.is_none() && line.contains("Week") {
                let line = line.trim();

                if let Some(slash_pos) = line.find('/') {
                    // find the space
                    if let Some(space_pos) = line.find(' ') {
                        let num = line[space_pos..slash_pos].trim();
                        week_num = Some(num.parse()?);
                    }
                }
            }

            match state {
                State::ScanMission => {
                    if found_mission(line) {
                        state = State::ReadMissionName;
                    }
                }
                State::ReadMissionName => {
                    name = get_name(line)?;
                    state = State::ScanUOD;
                }
                State::ScanUOD => {
                    if found_uod(line) {
                        state = State::ReadUOD;
                    }
                }
                State::ReadUOD => {
                    uod = get_uod(line);
                    state = State::ScanLocation;
                }
                State::ScanLocation => {
                    if found_location(line) {
                        state = State::ReadLocation;
                    }
                }
                State::ReadLocation => {
                    location = get_location(line)?;
                    state = State::ActivityParseComplete;
                }
                State::ActivityParseComplete => {
                    result.push(parse_activity(name, uod, location)?);

                    state = State::ScanMission;
                }
            }
        }

        if state != State::ScanMission {
            return Err(OpordParserError::IncompleteParse);
        }

        if let Some(week_num) = week_num {
            return Ok(ParsedOpord::new(week_num, result));
        }

        Err(OpordParserError::InvalidWeekFormat)
    }
}

fn parse_activity<'a>(
    name: &'a str,
    uod: &'a str,
    location: &'a str,
) -> Result<ActivityType<'a>, OpordParserError> {
    let details = ActivityDetails::new(location, uod);

    if name.contains("Leadership Laboratory") {
        let audience;

        if name.starts_with("Make Up") || name.starts_with("Make-Up") {
            return Ok(ActivityType::MULLAB(details));
        }

        if name.starts_with("GMC") {
            audience = LabAudience::GMC;
        } else if name.starts_with("POC") {
            audience = LabAudience::POC;
        } else if name.starts_with("Joint") {
            audience = LabAudience::Joint
        } else {
            return Err(OpordParserError::InvalidLLABAudience(name.to_string()));
        }

        return Ok(ActivityType::LLAB(audience, details));
    }

    if name.contains("Physical Training") {
        let pt_day;

        if name.starts_with("Monday/Tuesday") {
            pt_day = PTDay::MT;
        } else if name.starts_with("Wednesday/Thursday") {
            pt_day = PTDay::WTH;
        } else if name.contains("Remedial") {
            pt_day = PTDay::Remedial;
        } else {
            return Err(OpordParserError::InvalidPTDay(name.to_string()));
        }

        return Ok(ActivityType::PT(pt_day, details));
    }

    Ok(ActivityType::Unknown(details))
}

fn found_mission(line: &str) -> bool {
    line == "1. Mission"
}

fn found_uod(line: &str) -> bool {
    line.trim() == "c. UOD"
}

fn get_location(line: &str) -> Result<&str, OpordParserError> {
    let x = line.trim();
    const LOC_STR: &str = "Main Location:";

    match x.find(LOC_STR) {
        Some(loc_pos) => Ok(x[loc_pos + LOC_STR.len()..].trim()),
        None => Ok(x),
    }
}

fn get_name(line: &str) -> Result<&str, OpordParserError> {
    let x = line.trim();

    if x.starts_with("Week") {
        match x.find('/') {
            Some(pos) => return Ok(x[pos + 1..].trim()),
            None => return Err(OpordParserError::WeekMisionNameParseFail(x.to_string())),
        }
    }

    Ok(x)
}

fn found_location(line: &str) -> bool {
    line.trim() == "d. Main Location"
}

fn get_uod(line: &str) -> &str {
    let x = line.trim();

    // Format is usually GMC: OCPs.
    // can also be just "OCPs" as in the case of some POC labs..
    if let Some(colon_pos) = x.find(':') {
        return x[colon_pos + 1..].trim();
    }

    x
}

#[cfg(test)]
mod tests {
    use crate::opord_parser::{found_location, found_uod, get_location, get_name, get_uod};

    #[test]
    fn test_get_uod() {
        assert_eq!(
            "White Shirt/Blues",
            get_uod(&"	GMC: White Shirt/Blues".to_string())
        );

        assert_eq!("CITS", get_uod(&"	GMC & POC: CITS".to_string()));
    }

    #[test]
    fn test_found_location() {
        assert!(found_location(&"	d. Main Location".to_string()));
        assert!(!found_location(&"POC: Blues".to_string()));
    }

    #[test]
    fn test_get_name() {
        assert_eq!(
            "Joint Leadership Laboratory",
            get_name(&"	Week 1/ Joint Leadership Laboratory".to_string()).unwrap()
        )
    }

    #[test]
    fn test_get_location() {
        assert_eq!(
            "Torg 2150",
            get_location(&"	Main Location: Torg 2150".to_string()).unwrap()
        );
    }

    #[test]
    fn test_found_uod() {
        assert!(found_uod("	c. UOD"));
        assert!(!found_uod("	24 Aug 2021/1530-1645"))
    }
}
