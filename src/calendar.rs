use crate::month::CalMonth;
use crate::day::CalDay;
use crate::assignments::Assignment;

use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Calendar {
    year: String,
    months: Vec<CalMonth>,
}

impl Calendar {
    
    pub fn create_default_calendar() -> Self {
        // this is just gross looking for testing, will be much smaller after other features work
        let assignment1: Assignment = Assignment::new(String::from("test one"), String::from("basics of biology"), String::from("01/01/24 10:00:00"));
        let assignment2: Assignment = Assignment::new(String::from("my meeting"), String::from("daily stand up"), String::from("01/01/24 08:00:00"));
        let assignment3: Assignment = Assignment::new(String::from("homework 3"), String::from("working on code"), String::from("01/02/24 11:00:00"));
        let assignment4: Assignment = Assignment::new(String::from("my meeting"), String::from("daily stand up"), String::from("01/02/24 14:00:00"));
        let assignment5: Assignment = Assignment::new(String::from("my feb meeting"), String::from("blah blah"), String::from("02/01/24 09:00:00"));
        let day_one: CalDay = CalDay {date: "01/01/24".to_string(), assignments: vec![assignment1, assignment2]};
        let day_two: CalDay = CalDay {date: "01/02/24".to_string(), assignments: vec![assignment3, assignment4]};
        let day_three: CalDay = CalDay {date: "02/01/24".to_string(), assignments: vec![assignment5]};
        let month_one: CalMonth = CalMonth {name: "January".to_string(), days: vec![day_one, day_two]};
        let month_two: CalMonth = CalMonth {name: "February".to_string(), days: vec![day_three]};
    
        let calendar: Calendar = Calendar {
            year: "2024".to_string(),
            months: vec![month_one, month_two],
        };
    
        calendar
    }
}