pub mod activity;
pub mod opord_parser;
pub mod parser_error;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        activity::{ActivityDetails, ActivityType, LabAudience, PTDay},
        opord_parser::OpordParser,
    };

    #[test]
    fn week1() {
        let parser = OpordParser::new(Path::new("Week 1.txt"));

        let parsed = parser.parse().expect("failed to parse.");

        let result = parsed.activities();

        let llab = ActivityType::LLAB(
            LabAudience::Joint,
            ActivityDetails::new("Torg 2150".to_string(), "White Shirt/Blues".to_string()),
        );

        let pt = ActivityType::PT(
            PTDay::WTH,
            ActivityDetails::new(
                "Drillfield".to_string(),
                "AF PTGs / VTCC PT Gear".to_string(),
            ),
        );

        let mullab = ActivityType::MULLAB(ActivityDetails::new(
            "Mil Bldg Rom 208".to_string(),
            "White Shirt/Blues".to_string(),
        ));

        let picnic = ActivityType::Unknown(ActivityDetails::new(
            "Blacksburg Municipal Park Shelter 9".to_string(),
            "CITS".to_string(),
        ));

        println!("{:#?}", result);

        assert_eq!(1, parsed.week_num());
        assert!(result.contains(&picnic));
        assert!(result.contains(&mullab));
        assert!(result.contains(&llab));
        assert!(result.contains(&pt));
    }
}
