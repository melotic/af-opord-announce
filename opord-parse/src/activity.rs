#[derive(PartialEq, Debug)]
pub struct ParsedOpord {
    week_num: u8,
    activities: Vec<ActivityType>,
}

impl ParsedOpord {
    pub fn new(week_num: u8, activities: Vec<ActivityType>) -> Self {
        Self {
            week_num,
            activities,
        }
    }

    /// Get a reference to the parsed opord's week num.
    pub fn week_num(&self) -> u8 {
        self.week_num
    }

    /// Get a reference to the parsed opord's activities.
    pub fn activities(&self) -> &[ActivityType] {
        self.activities.as_slice()
    }
}

#[derive(PartialEq, Debug)]
pub enum ActivityType {
    Unknown(ActivityDetails),
    PT(PTDay, ActivityDetails),
    LLAB(LabAudience, ActivityDetails),
    MULLAB(ActivityDetails),
}

#[derive(PartialEq, Debug)]
pub enum LabAudience {
    GMC,
    POC,
    Joint,
}

#[derive(PartialEq, Debug)]
pub enum PTDay {
    MT,
    WTH,
}

#[derive(PartialEq, Debug)]
pub struct ActivityDetails {
    location: String,
    uniform: String,
}

impl ActivityDetails {
    pub fn new(location: String, uniform: String) -> Self {
        Self { location, uniform }
    }

    /// Get a reference to the activity details's location.
    pub fn location(&self) -> &str {
        self.location.as_str()
    }

    /// Get a reference to the activity details's uniform.
    pub fn uniform(&self) -> &str {
        self.uniform.as_str()
    }
}
