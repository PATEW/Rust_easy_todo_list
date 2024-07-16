use crate::item::{Item, ItemType, KanbanStatus};

use time::macros::datetime;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Calendar {
    pub assignments: Vec<Item>,
}

impl Calendar {
    
    pub fn create_default_calendar() -> Self {
        // this is just gross looking for testing, will be much smaller after other features work
        let assignment1 = Item::new(
            String::from("test one"),
            ItemType::Assignment,
            String::from("basics of biology"),
            datetime!(2024-07-16 10:00:00),
            datetime!(2024-07-16 11:00:00),
            KanbanStatus::Todo
        );
        let assignment2: Item = Item::new(
            String::from("my meeting"),
            ItemType::Meeting,
            String::from("daily stand up"),
            datetime!(2024-07-16 12:00:00),
            datetime!(2024-07-16 13:00:00),
            KanbanStatus::None
        );
        let assignment3: Item = Item::new(
            String::from("homework 3"),
            ItemType::Assignment,
            String::from("working on code"),
            datetime!(2024-07-17 08:00:00),
            datetime!(2024-07-17 09:00:00),
            KanbanStatus::InProgress
        );
        let assignment4: Item = Item::new(
            String::from("my meeting"),
            ItemType::Meeting,
            String::from("daily stand up"),
            datetime!(2024-08-02 09:00:00),
            datetime!(2024-08-01 13:00:00),
            KanbanStatus::Done
        );
        let assignment5: Item = Item::new(
            String::from("project requirements"),
            ItemType::Assignment,
            String::from("blah blah"),
            datetime!(2024-08-07 12:00:00),
            datetime!(2024-08-07 15:00:00),
            KanbanStatus::Backburner
        );

        let calendar: Calendar = Calendar {
            assignments: vec![assignment1, assignment2, assignment3, assignment4, assignment5],
        };
    
        calendar
    }

    // pub fn add_assignment(details: (String, String, PrimitiveDateTime, PrimitiveDateTime)) -> Assignment {
    //     let assignment =  Assignment::new(String, String, datetime!(2024-01-01 10:00:00), datetime!(2024-01-01 11:00:00));
    // }
}