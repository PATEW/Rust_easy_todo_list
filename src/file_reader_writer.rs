
use std::{fs::File, io::{BufWriter, Write}, path::Path};

use crate::{app::App, calendar::Calendar};

pub trait DataIO {
    fn load_data() -> Calendar;
    fn save_data(&self);
}

impl DataIO for App {
    fn load_data() -> Calendar {
        let calendar: Calendar = load_from_json();
        calendar
    }
    
    fn save_data(&self) {
        write_to_json(&self.user_data.user_calendar);
    }
}


fn load_from_json() -> Calendar {
    if Path::new("./data/data.json").is_file() {
        let calendar: Calendar = Calendar::create_default_calendar(); // replace this with loading data
        calendar
    } else {
        let calendar: Calendar = Calendar::create_default_calendar(); //create new one with default data
        calendar
    }
}

fn write_to_json(calendar: &Calendar) -> std::io::Result<()> {

    let json_str = match serde_json::to_string(calendar) {
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