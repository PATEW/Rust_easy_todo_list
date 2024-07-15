use std::error;
use chrono::{Local, Datelike, NaiveDate, Duration};
use crate::calendar::Calendar;
use crate::file_reader_writer::DataIO;

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
    pub user_calendar: Calendar,
    pub list_of_days_in_selected_month: Vec<(u32, bool)>, // (day, is_current_month)
    pub todays_day_month_year: (String, String, String),
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let loaded_data: Calendar = App::load_data();
        let todays_day_month_year: (String, String, String) = App::get_todays_day_month_year();
        let list_of_days_in_selected_month = App::get_list_of_days_in_current_month();
        Self {
            running: true,
            counter: 0,
            focused_chunk: 0,
            table_selected_cell: (0, 0),
            user_calendar: loaded_data,
            list_of_days_in_selected_month,
            todays_day_month_year,
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

    pub fn get_todays_day_month_year() -> (String, String, String) {
        let today = Local::now().date_naive();
        
        let year = today.year().to_string();
        let day = today.day().to_string();
        
        let month = match today.month() {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => unreachable!(),
        }.to_string();

        (day, month, year)
    }

    pub fn get_list_of_days_in_current_month() -> Vec<(u32, bool)> {
        let today = Local::now().date_naive();
        let year = today.year();
        let month = today.month();
    
        let first_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let last_of_month = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap() - Duration::days(1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap() - Duration::days(1)
        };
    
        let first_weekday = first_of_month.weekday();
        let mut current_date = first_of_month - Duration::days(first_weekday.num_days_from_sunday() as i64);
    
        let mut days = Vec::with_capacity(42);
    
        while days.len() < 42 {
            days.push((current_date.day(), current_date.month() == month));
            if current_date == last_of_month && days.len() >= 28 {
                // If we've reached the last day of the month and have at least 4 weeks,
                // fill the rest with days from the next month
                current_date += Duration::days(1);
                while days.len() < 42 {
                    days.push((current_date.day(), false));
                    current_date += Duration::days(1);
                }
                break;
            }
            current_date += Duration::days(1);
        }
    
        days
    }
}
