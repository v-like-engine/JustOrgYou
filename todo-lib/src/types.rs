use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Priority indicator. Lower number = higher priority (0 is highest)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Priority(pub u8);

impl Priority {
    pub fn new(value: u8) -> Self {
        Priority(value)
    }

    /// Convert to letter representation (0=A, 1=B, 2=C, etc.)
    pub fn to_letter(&self) -> char {
        (b'A' + self.0) as char
    }

    /// Create from letter (A=0, B=1, C=2, etc.)
    pub fn from_letter(letter: char) -> Option<Self> {
        if letter.is_ascii_uppercase() {
            Some(Priority((letter as u8) - b'A'))
        } else {
            None
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_letter())
    }
}

/// Task state keyword
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Keyword {
    Todo,
    Done,
    Inbox,
    Waiting,
    Someday,
    Custom(String),
}

impl Keyword {
    pub fn as_str(&self) -> &str {
        match self {
            Keyword::Todo => "TODO",
            Keyword::Done => "DONE",
            Keyword::Inbox => "INBOX",
            Keyword::Waiting => "WAITING",
            Keyword::Someday => "SOMEDAY",
            Keyword::Custom(s) => s.as_str(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "TODO" => Keyword::Todo,
            "DONE" => Keyword::Done,
            "INBOX" => Keyword::Inbox,
            "WAITING" => Keyword::Waiting,
            "SOMEDAY" => Keyword::Someday,
            _ => Keyword::Custom(s.to_string()),
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(self, Keyword::Done)
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Repetition pattern for periodic tasks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepeatPattern {
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Custom { interval: u32, unit: TimeUnit },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeUnit {
    Day,
    Week,
    Month,
    Year,
}

/// Periodic date time for scheduling and deadlines
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeriodicDateTime {
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub repeat: Option<RepeatPattern>,
}

impl PeriodicDateTime {
    pub fn new(date: NaiveDate, time: Option<NaiveTime>) -> Self {
        PeriodicDateTime {
            date,
            time,
            repeat: None,
        }
    }

    pub fn with_repeat(mut self, repeat: RepeatPattern) -> Self {
        self.repeat = Some(repeat);
        self
    }

    pub fn is_past(&self) -> bool {
        let now = Local::now().naive_local();
        if let Some(time) = self.time {
            self.date.and_time(time) < now
        } else {
            self.date < now.date()
        }
    }

    pub fn next_occurrence(&self) -> Option<PeriodicDateTime> {
        self.repeat.as_ref().map(|pattern| {
            let next_date = match pattern {
                RepeatPattern::Daily => self.date + chrono::Duration::days(1),
                RepeatPattern::Weekly => self.date + chrono::Duration::weeks(1),
                RepeatPattern::Monthly => {
                    // Add one month
                    if self.date.month() == 12 {
                        NaiveDate::from_ymd_opt(self.date.year() + 1, 1, self.date.day())
                            .unwrap_or(self.date)
                    } else {
                        NaiveDate::from_ymd_opt(
                            self.date.year(),
                            self.date.month() + 1,
                            self.date.day(),
                        )
                        .unwrap_or(self.date)
                    }
                }
                RepeatPattern::Yearly => {
                    NaiveDate::from_ymd_opt(
                        self.date.year() + 1,
                        self.date.month(),
                        self.date.day(),
                    )
                    .unwrap_or(self.date)
                }
                RepeatPattern::Custom { interval, unit } => match unit {
                    TimeUnit::Day => self.date + chrono::Duration::days(*interval as i64),
                    TimeUnit::Week => self.date + chrono::Duration::weeks(*interval as i64),
                    TimeUnit::Month => {
                        let months = self.date.month() + interval;
                        let years = months / 12;
                        let month = months % 12;
                        NaiveDate::from_ymd_opt(
                            self.date.year() + years as i32,
                            month,
                            self.date.day(),
                        )
                        .unwrap_or(self.date)
                    }
                    TimeUnit::Year => NaiveDate::from_ymd_opt(
                        self.date.year() + *interval as i32,
                        self.date.month(),
                        self.date.day(),
                    )
                    .unwrap_or(self.date),
                },
            };

            PeriodicDateTime {
                date: next_date,
                time: self.time,
                repeat: self.repeat.clone(),
            }
        })
    }
}

impl fmt::Display for PeriodicDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(time) = self.time {
            write!(f, "{} {}", self.date, time)?;
        } else {
            write!(f, "{}", self.date)?;
        }

        if let Some(repeat) = &self.repeat {
            write!(f, " (repeats: {:?})", repeat)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_conversion() {
        let p = Priority::new(0);
        assert_eq!(p.to_letter(), 'A');

        let p = Priority::from_letter('B').unwrap();
        assert_eq!(p.0, 1);
    }

    #[test]
    fn test_keyword_parsing() {
        assert_eq!(Keyword::from_str("TODO"), Keyword::Todo);
        assert_eq!(Keyword::from_str("done"), Keyword::Done);
        assert!(Keyword::Done.is_done());
    }
}
