mod assignments;
use std::{fs::File, io::{BufWriter, Write}, vec};
use serde_derive::Serialize;

use assignments::Assignment;

#[derive(Debug)]
enum AppState {
    StartUp,
    MainMenu,
    CalendarView,
    CreateItem,
    ModifyItem,
    DeleteItem,
    SaveandQuit,
}

#[derive(Serialize)]
struct CalDay {
    date: String,
    assignments: Vec<Assignment>,
}

#[derive(Serialize)]
struct CalMonth {
    name: String,
    days: Vec<CalDay>,
}

#[derive(Serialize)]
struct Calendar {
    year: String,
    months: Vec<CalMonth>,
}

fn main() {
    let mut current_state: AppState = AppState::StartUp;

    // Populate the calendar with previous info, if there is any

    // if calendar exists, use it
    // else create new one:
    create_default_calendar();


    current_state = AppState::MainMenu;

    println!("\n_________________\nTo-Do list app\n_________________\npick an option:\n");
    println!("1. Calendar View\n2. Create Item\n3. Modify Item\n4. Delete Item\n5. Save and Quit\n");

    let mut valid_option: bool = false; 
    while !valid_option {
        let response: i32 = read_int();
        if response >= 1 && response <=5 {

            valid_option = true;

            match response {
                1 => current_state = AppState::CalendarView,
                2 => {current_state = AppState::CreateItem; create_item()},
                3 => current_state = AppState::ModifyItem,
                4 => current_state = AppState::DeleteItem,
                5 => current_state = AppState::SaveandQuit,
                _ => panic!("option selection should not be possible"),
            }

            println!("\nyou picked option {} which is {:?}\n", response, current_state);
        }
    }
}

pub fn create_item() {
    println!("create item here");
    // let assignment1 = Assignment::new(String::from("my homework"), 1);
    // let (ass_name, ass_num) = assignment1.get_info();
    // println!("assignment {} created: {}", ass_num, ass_name);
}

pub fn create_default_calendar() {
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

    let json_str = match serde_json::to_string(&calendar) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Unable to load data");
            panic!("JSON brokey or something");
        }
    };

    println!("loaded calendar: {}", json_str);
    let print_result: std::io::Result<()> = write_to_json(json_str);
}

fn write_to_json(json_str: String) -> std::io::Result<()> {
    let file = File::create("data.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &json_str)?;
    writer.flush()?;
    Ok(())
}

fn read_int() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().parse().unwrap()
}