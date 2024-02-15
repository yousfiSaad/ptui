
use std::{fs, u128, usize};

use chrono::{NaiveDateTime, Datelike};

const DEFAULT_DATA_FILE: &str = "data/prayer_times.txt";
const DEFAULT_DATA_FILE_ENV_VAR_NAME: &str = "DATA_FILE";

/// Application.
#[derive(Debug, Default)]
pub struct PrayerTime {
    pub time: u128,
    pub name: String
}

#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,

    //todo: change to unums
    pub show_remaining_time: bool,

    pub show_fasting_time: bool,

    pub now: u128,

    pub prayer_times: Vec<PrayerTime>,

    // current time
    pub ipos: usize,

    pub previous_icha: usize,
}

fn parse_line(str :&str) -> PrayerTime {
    let mut parts = str.split(" ");

    let pt = PrayerTime {
        time: parts.next()
            .unwrap()
            .to_string().parse::<u128>()
            .unwrap(), 
        name: parts.next()
            .unwrap()
            .to_string() 
    };

    return pt;
}

fn is_in_today(timestamp: u128) -> bool {
    let date_time = NaiveDateTime::from_timestamp_millis(timestamp as i64);
    return date_time.unwrap().day() == chrono::Local::now().day();
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut s = Self::default();
        let data_file = std::env::var(DEFAULT_DATA_FILE_ENV_VAR_NAME.to_string())
            .unwrap_or(DEFAULT_DATA_FILE.to_string());
        if let Ok(content) = fs::read_to_string(&data_file){
            s.prayer_times = content.split("\n").filter(|x| x.chars().count() > 0)
            .map(|str| parse_line(&str))
            .collect::<Vec<PrayerTime>>();
        } else {
            s.should_quit = true;
            println!("Data file '{}' not found, see Readme.md to generate data file according to your location", data_file);
        }
        s.show_remaining_time = true;
        return s;
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        let start = std::time::SystemTime::now();
        let since_the_epoch = start
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.now = since_the_epoch;
        self.ipos = self.pos();
        self.previous_icha = self.ipos;
        while  is_in_today(self.prayer_times[self.ipos + 1].time) && is_in_today(self.prayer_times[self.previous_icha].time) {
            self.previous_icha -= 1;
        }
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn toggle_fastring_time(&mut self){
        self.show_fasting_time = !self.show_fasting_time;
    }

    pub fn toggle_time_duration(&mut self){
        self.show_remaining_time = !self.show_remaining_time;
    }

    fn pos(&self) -> usize {
        let mut ipos = 0;

        let mut i:usize = 0;
        while i < self.prayer_times.len() && self.now > self.prayer_times[i].time {
            ipos = i;
            i += 1;
        }

        return ipos;
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
}
