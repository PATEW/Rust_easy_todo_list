use std::error;

use chrono::{Local, Datelike};

use crate::calendar::Calendar;
use crate::file_reader_writer::{self, DataIO};
use crate::user_data::UserData;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.

pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    /// focused chunk (0 for top, 1 for bottom)
    pub focused_chunk: usize,
    /// selected cell in the table
    pub table_selected_cell: (usize, usize),

    pub user_data: UserData, // data to be changed during app use

    pub todays_date: (u16, u8, u8),
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let loaded_data: Calendar = App::load_data();
        let todays_date: (u16, u8, u8) = App::get_todays_year_month_day();
        Self::initialize(loaded_data, todays_date)
    }

    // set fields
    fn initialize(loaded_data: Calendar, todays_date: (u16, u8, u8)) -> Self {
        let new_data: UserData = UserData::new(loaded_data);
        Self {
            running: true,
            counter: 0,
            focused_chunk: 0,
            table_selected_cell: (0, 0),
            user_data: new_data,
            todays_date: todays_date,
        }
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

    pub fn get_todays_year_month_day() -> (u16, u8, u8) {
        let today = Local::now().date_naive();
        (today.year() as u16, today.month() as u8, today.day() as u8)
    }
}
