use std::{error, path::Path};

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
    /// save data to load
    pub save_data: Calendar,
}

impl App {
    fn initialize(previous_data: Calendar) -> Self {
        Self {
            running: true,
            counter: 0,
            focused_chunk: 0,
            table_selected_cell: (0, 0),
            save_data: previous_data,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let previous_data = App::load_data();
        Self::initialize(previous_data)
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

    pub fn load_data() -> Calendar {
        let mut current_state: AppState = AppState::StartUp;

        if Path::new("./data/data.json").is_file() {
            let calendar: Calendar = Calendar::create_default_calendar(); // replace this with loading data
            calendar
        } else {
            let calendar: Calendar = Calendar::create_default_calendar(); //create new one with default data
            calendar
        }
    }

    pub fn create_item() {
        println!("\nPlease enter the event name: ");
        println!("\nPlease enter the event description:\n");
        println!("\nWhen does the event start? Please enter it in the format 'YYYY-MM-DD HH:MM'\n");
        println!("\nWhen does the event end? Please enter it in the format 'YYYY-MM-DD HH:MM'\n");
    }
    
    fn write_to_json(save_data: Calendar) -> std::io::Result<()> {

        let json_str = match serde_json::to_string(&save_data) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Unable to load data");
                panic!("JSON brokey or something");
            }
        };

        let file = File::create("data/save_data.json")?;
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

    pub fn save_new_data(save_data: Calendar) {
        App::write_to_json(save_data);
    }
}
