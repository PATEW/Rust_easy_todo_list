use std::{io::BufWriter, path::Path};

use crate::calendar::Calendar;

pub struct UserData {
    pub user_calendar: Calendar,
}

impl UserData {
    pub fn new(previous_saved_calendar_data: Calendar) -> Self {
        UserData {user_calendar: previous_saved_calendar_data}
    }

}