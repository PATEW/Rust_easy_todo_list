mod calendar;

use calendar::Calendar;
use std::{fs::File, io::{BufWriter, Write}, vec};
use serde_derive::Serialize;

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


fn main() {
    let mut current_state: AppState = AppState::StartUp;

    // Populate the calendar with previous info, if there is any

    // if calendar exists, use it
    // else create new one:
    let calendar: Calendar = calendar::create_default_calendar();

    let json_str = match serde_json::to_string(&calendar) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Unable to load data");
            panic!("JSON brokey or something");
        }
    };

    println!("loaded calendar: {}", json_str);
    let print_result: std::io::Result<()> = write_to_json(json_str);

    //////////////////////////////////////////

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