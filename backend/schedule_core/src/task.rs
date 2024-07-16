/// all time is in utc
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub time: Option<NaiveTime>, // set: notice
    pub date: Option<NaiveDate>, // unset: notice everyday
    pub category: Option<String>, // None is default
    pub priority: Priority, // Default is default
    // status: Status,
}

use chrono::{Datelike, Days, NaiveDate, NaiveTime, Weekday};

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Loop {
    OneOff{date: NaiveDate},
    EveryWeek{date: NaiveDate, weekday: Weekday}, // with last date
    Everyday{date: NaiveDate},
}

impl Loop {
    pub fn matches(&self, date: NaiveDate)->bool {
        use Loop::*;
        match *self {
            OneOff{date:d} => d==date,
            EveryWeek{date:_, weekday: wd} => wd==date.weekday(),
            Everyday{date:ld} => date>=ld,
        }
    }
    pub fn next(&self)->NaiveDate {
        use Loop::*;
        match *self {
            OneOff{date:d} => d,
            EveryWeek{date:ld, weekday: wd} => 
                ld.checked_add_days(
                    Days::new(7-ld.weekday().days_since(wd) as u64)
                ).unwrap(),
            Everyday{date:ld} => ld
        }
    }
    pub fn next_since(&self, date: NaiveDate)->Option<NaiveDate> {
        use Loop::*;
        let next = self.next();
        match *self {
            OneOff{date:d} => if d>date {None} else {Some(d)},
            EveryWeek{date:ld, weekday: wd} => 
                if ld>date {Some(next)}
                else {date.checked_add_days(
                    Days::new(wd.days_since(date.weekday()) as u64)
                )},
            Everyday{date:ld} => if ld>date {Some(next)} else {Some(date)}
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Default,
    Low,
    Mid,
    High,
}

// TODO! status
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum _Status {
    Created,
    Finished,
    Abort,
}

