use crate::assignments::Assignment;

use time::macros::datetime;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Calendar {
    pub assignments: Vec<Assignment>,
}

impl Calendar {
    
    pub fn create_default_calendar() -> Self {
        // this is just gross looking for testing, will be much smaller after other features work
        let assignment1: Assignment = Assignment::new(String::from("test one"), String::from("basics of biology"), datetime!(2024-01-01 10:00:00), datetime!(2024-01-01 11:00:00));
        let assignment2: Assignment = Assignment::new(String::from("my meeting"), String::from("daily stand up"), datetime!(2024-01-01 12:00:00), datetime!(2024-01-01 13:00:00));
        let assignment3: Assignment = Assignment::new(String::from("homework 3"), String::from("working on code"), datetime!(2024-01-02 08:00:00), datetime!(2024-01-01 09:00:00));
        let assignment4: Assignment = Assignment::new(String::from("my meeting"), String::from("daily stand up"), datetime!(2024-01-02 09:00:00), datetime!(2024-01-01 13:00:00));
        let assignment5: Assignment = Assignment::new(String::from("my feb meeting"), String::from("blah blah"), datetime!(2024-01-01 12:00:00), datetime!(2024-01-01 15:00:00));

        let calendar: Calendar = Calendar {
            assignments: vec![assignment1, assignment2, assignment3, assignment4, assignment5],
        };
    
        calendar
    }

    // pub fn add_assignment(details: (String, String, PrimitiveDateTime, PrimitiveDateTime)) -> Assignment {
    //     let assignment =  Assignment::new(String, String, datetime!(2024-01-01 10:00:00), datetime!(2024-01-01 11:00:00));
    // }
}