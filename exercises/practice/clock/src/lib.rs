use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_min = (hours*60 + minutes)%1440;
        if total_min<0{
            total_min = 1440 + total_min
        }
        Self{hours: total_min/60, minutes: total_min%60}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {


        Clock::new(self.hours, self.minutes+minutes)
    }
/*
    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
*/
}

impl Display for Clock{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
