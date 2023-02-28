use std::cmp::Ordering;
use std::fmt;

pub struct TimeRemaining {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
}

impl TimeRemaining {
    pub fn is_late(&self) -> bool {
        self.days <= 0 && self.hours <= 0 && self.minutes < 0
    }
}

impl PartialOrd for TimeRemaining {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeRemaining {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.days > other.days {
            Ordering::Greater
        } else if self.days < other.days {
            Ordering::Less
        } else {
            if self.hours > other.hours {
                Ordering::Greater
            } else if self.hours < other.hours {
                Ordering::Less
            } else {
                if self.minutes > other.minutes {
                    Ordering::Greater
                } else if self.minutes < other.minutes {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

impl PartialEq for TimeRemaining {
    fn eq(&self, other: &Self) -> bool {
        self.days == other.days && self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Eq for TimeRemaining {}

impl fmt::Display for TimeRemaining {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let days = if self.days != 1 { "days" } else { "day" };
        let hours = if self.hours != 1 { "hours" } else { "hour" };
        let minutes = if self.minutes != 1 {
            "minutes"
        } else {
            "minute"
        };

        write!(
            f,
            "{} {}, {} {}, {} {}",
            self.days, days, self.hours, hours, self.minutes, minutes
        )
    }
}
