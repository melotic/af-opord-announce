pub mod activity;
pub mod opord_parser;
pub mod parser_error;
pub mod week_msg;

#[cfg(test)]
mod tests {

    use crate::{
        activity::{ActivityDetails, ActivityType, LabAudience, PTDay},
        opord_parser::OpordParser,
    };

    #[test]
    fn week1() {
        let opord = std::fs::read_to_string("Week 1.txt").expect("err opening opord");
        let parser = OpordParser::new(&opord);
        let parsed = parser.parse().expect("failed to parse.");

        let result = parsed.activities();

        let llab = ActivityType::LLAB(
            LabAudience::Joint,
            ActivityDetails::new("Torg 2150", "White Shirt/Blues"),
        );

        let pt = ActivityType::PT(
            PTDay::WTH,
            ActivityDetails::new("Drillfield", "AF PTGs / VTCC PT Gear"),
        );

        let mullab = ActivityType::MULLAB(ActivityDetails::new(
            "Mil Bldg Rom 208",
            "White Shirt/Blues",
        ));

        let picnic = ActivityType::Unknown(ActivityDetails::new(
            "Blacksburg Municipal Park Shelter 9",
            "CITS",
        ));

        println!("{:#?}", result);

        assert_eq!(1, parsed.week_num());
        assert!(result.contains(&picnic));
        assert!(result.contains(&mullab));
        assert!(result.contains(&llab));
        assert!(result.contains(&pt));
    }
}
