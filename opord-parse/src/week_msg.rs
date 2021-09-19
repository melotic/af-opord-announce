use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct WeekMsg {
    week_num: u8,
    opord_link: String,
    countdowns: Vec<Countdown>,
    activities: Vec<Activity>,
    announcements: Vec<Announcement>,
}

impl WeekMsg {
    pub fn new(
        week_num: u8,
        opord_link: String,
        countdowns: Vec<Countdown>,
        activities: Vec<Activity>,
        announcements: Vec<Announcement>,
    ) -> Self {
        Self {
            week_num,
            opord_link,
            countdowns,
            activities,
            announcements,
        }
    }

    /// Get a mutable reference to the week msg's announcements.
    pub fn announcements_mut(&mut self) -> &mut Vec<Announcement> {
        &mut self.announcements
    }

    /// Get a reference to the week msg's announcements.
    pub fn announcements(&self) -> &[Announcement] {
        self.announcements.as_slice()
    }

    /// Get a mutable reference to the week msg's countdowns.
    pub fn countdowns_mut(&mut self) -> &mut Vec<Countdown> {
        &mut self.countdowns
    }

    /// Get a reference to the week msg's countdowns.
    pub fn countdowns(&self) -> &[Countdown] {
        self.countdowns.as_slice()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Countdown {
    text: String,
    date: String
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Announcement {
    title: String,
    subtitle: String,
    content: String,
}
