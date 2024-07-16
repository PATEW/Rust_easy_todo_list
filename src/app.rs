use std::error;
use chrono::{Local, Datelike, NaiveDate, Duration, Weekday};
use crate::calendar::Calendar;
use crate::file_reader_writer::DataIO;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalendarView {
    Year,
    Month,
    Week,
    Day,
}

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
    pub currently_selected_date: NaiveDate,
    pub current_view: CalendarView,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let loaded_data: Calendar = App::load_data();
        let todays_day_month_year: (String, String, String) = App::get_todays_day_month_year();
        let today = Local::now().date_naive();
        let list_of_days_in_selected_month = App::get_list_of_days_for_month(today.year(), today.month());

        Self {
            running: true,
            counter: 0,
            focused_chunk: 0,
            table_selected_cell: (0, 0),
            user_calendar: loaded_data,
            list_of_days_in_selected_month,
            todays_day_month_year,
            currently_selected_date: today,
            current_view: CalendarView::Month,
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

    pub fn get_list_of_days_for_month(year: i32, month: u32) -> Vec<(u32, bool)> {
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

    pub fn update_selected_date(&mut self, date: NaiveDate) {
        self.currently_selected_date = date;
        self.list_of_days_in_selected_month = App::get_list_of_days_for_month(date.year(), date.month());
    }

    pub fn month_string_to_number(month: &str) -> u32 {
        match month {
            "January" => 1,
            "February" => 2,
            "March" => 3,
            "April" => 4,
            "May" => 5,
            "June" => 6,
            "July" => 7,
            "August" => 8,
            "September" => 9,
            "October" => 10,
            "November" => 11,
            "December" => 12,
            _ => 0,
        }
    }

    pub fn zoom_in(&mut self) {
        self.current_view = match self.current_view {
            CalendarView::Year => CalendarView::Month,
            CalendarView::Month => CalendarView::Week,
            CalendarView::Week => CalendarView::Day,
            CalendarView::Day => CalendarView::Day, // No change if already at day view
        };
    }

    pub fn zoom_out(&mut self) {
        self.current_view = match self.current_view {
            CalendarView::Day => CalendarView::Week,
            CalendarView::Week => CalendarView::Month,
            CalendarView::Month => CalendarView::Year,
            CalendarView::Year => CalendarView::Year, // No change if already at year view
        };
    }

    pub fn navigate_previous(&mut self) {
        match self.current_view {
            CalendarView::Year => {
                self.currently_selected_date = self.currently_selected_date.with_year(self.currently_selected_date.year() - 1).unwrap();
            },
            CalendarView::Month => {
                self.currently_selected_date = self.currently_selected_date.checked_sub_months(chrono::Months::new(1)).unwrap();
            },
            CalendarView::Week => {
                self.currently_selected_date -= Duration::weeks(1);
            },
            CalendarView::Day => {
                // Do nothing for day view
            },
        }
        self.update_selected_date(self.currently_selected_date);
    }

    pub fn navigate_next(&mut self) {
        match self.current_view {
            CalendarView::Year => {
                self.currently_selected_date = self.currently_selected_date.with_year(self.currently_selected_date.year() + 1).unwrap();
            },
            CalendarView::Month => {
                self.currently_selected_date = self.currently_selected_date.checked_add_months(chrono::Months::new(1)).unwrap();
            },
            CalendarView::Week => {
                self.currently_selected_date += Duration::weeks(1);
            },
            CalendarView::Day => {
                // Do nothing for day view
            },
        }
        self.update_selected_date(self.currently_selected_date);
    }

    pub fn get_current_view_data(&self) -> Vec<(String, bool)> {
        match self.current_view {
            CalendarView::Year => {
                (1..=12).map(|month| {
                    let month_name = match month {
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
                    };
                    (month_name.to_string(), month == self.currently_selected_date.month())
                }).collect()
            },
            CalendarView::Month => {
                self.list_of_days_in_selected_month.iter()
                    .map(|&(day, is_current_month)| (day.to_string(), is_current_month))
                    .collect()
            },
            CalendarView::Week => {
                let week_start = self.currently_selected_date - Duration::days(self.currently_selected_date.weekday().num_days_from_monday() as i64);
                (0..7).map(|offset| {
                    let date = week_start + Duration::days(offset);
                    (format!("{} {}", date.format("%a"), date.day()), date.month() == self.currently_selected_date.month())
                }).collect()
            },
            CalendarView::Day => {
                vec![(self.currently_selected_date.format("%A, %B %d, %Y").to_string(), true)]
            },
        }
    }
}