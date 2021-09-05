use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct WeekMsg {
    week_num: u8,
    opord_link: String,
    activities: Vec<Activity>,
    announcements: Vec<Announcement>,
}

impl WeekMsg {
    pub fn new(
        week_num: u8,
        opord_link: String,
        activities: Vec<Activity>,
        announcements: Vec<Announcement>,
    ) -> Self {
        Self {
            week_num,
            opord_link,
            activities,
            announcements,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    name: String,
    event: String,
    location: String,
    uniform: String,
}

impl Activity {
    pub fn new(name: String, event: String, location: String, uniform: String) -> Self {
        Self {
            name,
            event,
            location,
            uniform,
        }
    }
}

impl Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ {} {} {} {} {} ]",
            self.name.red(),
            "@".white().bold(),
            self.location.green(),
            "in".white().bold(),
            self.uniform.blue()
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct Announcement {
    title: String,
    subtitle: String,
    content: String,
}