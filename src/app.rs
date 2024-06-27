use std::error;

use crate::calendar::Calendar;
use std::{fs::File, io::{BufWriter, Write}, vec};
use serde_derive::Serialize;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
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

pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    /// focused chunk (0 for top, 1 for bottom)
    pub focused_chunk: usize,
    /// selected cell in the table
    pub table_selected_cell: (usize, usize),
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            focused_chunk: 0,
            table_selected_cell: (0, 0),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        App::run();
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn run() {
        let mut current_state: AppState = AppState::StartUp;

        // Populate the calendar with previous info, if there is any

        // if calendar exists, use it
        // else create new one:
        let calendar: Calendar = Calendar::create_default_calendar();

        let json_str = match serde_json::to_string(&calendar) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Unable to load data");
                panic!("JSON brokey or something");
            }
        };

        println!("loaded calendar: {}", json_str);
        let _print_result: std::io::Result<()> = App::write_to_json(json_str);

        //////////////////////////////////////////

        current_state = AppState::MainMenu;

        println!("\n_________________\nTo-Do list app\n_________________\npick an option:\n");
        println!("1. Calendar View\n2. Create Item\n3. Modify Item\n4. Delete Item\n5. Save and Quit\n");

        let mut valid_option: bool = false; 
        while !valid_option {
            let response: i32 = App::read_int();
            if response >= 1 && response <=5 {

                valid_option = true;

                match response {
                    1 => current_state = AppState::CalendarView,
                    2 => {current_state = AppState::CreateItem; App::create_item()},
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
        println!("\nPlease enter the event name: ");
        println!("\nPlease enter the event description:\n");
        println!("\nWhen does the event start? Please enter it in the format 'YYYY-MM-DD HH:MM'\n");
        println!("\nWhen does the event end? Please enter it in the format 'YYYY-MM-DD HH:MM'\n");
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
}
