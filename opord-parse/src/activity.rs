#[derive(PartialEq, Debug)]
pub struct ParsedOpord<'a> {
    week_num: u8,
    activities: Vec<ActivityType<'a>>,
}

impl<'a> ParsedOpord<'a> {
    pub fn new(week_num: u8, activities: Vec<ActivityType<'a>>) -> Self {
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
pub enum ActivityType<'a> {
    Unknown(ActivityDetails<'a>),
    PT(PTDay, ActivityDetails<'a>),
    LLAB(LabAudience, ActivityDetails<'a>),
    MULLAB(ActivityDetails<'a>),
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
pub struct ActivityDetails<'a> {
    location: &'a str,
    uniform: &'a str,
}

impl<'a> ActivityDetails<'a> {
    pub fn new(location: &'a str, uniform: &'a str) -> Self {
        Self { location, uniform }
    }

    /// Get a reference to the activity details's location.
    pub fn location(&self) -> &str {
        self.location
    }

    /// Get a reference to the activity details's uniform.
    pub fn uniform(&self) -> &str {
        self.uniform
    }
}
