
use std::{fs::{self, File}, io::BufWriter, path::Path};

use crate::{app::App, calendar::Calendar};

const SAVE_DATA_PATH: &str = "./data/save_data.json";

pub trait DataIO {
    fn load_data() -> Calendar;
    fn save_data(&self);
}

impl DataIO for App {
    fn load_data() -> Calendar {
        load_from_json()
    }
    
    fn save_data(&self) {
        match write_to_json(&self.user_calendar) {
            Ok(_) => println!("Data saved successfully"),
            Err(e) => println!("COULD NOT SAVE DATA!!!\n{}", e),
        }
    }
}


fn load_from_json() -> Calendar {
    if Path::new(SAVE_DATA_PATH).is_file() {
        let path = SAVE_DATA_PATH;
        match fs::read_to_string(path) {
            Ok(data) => {
                match serde_json::from_str(&data) {
                    Ok(calendar) => {
                        println!("Created calendar from saved json file.");
                        calendar
                    },
                    Err(e) => {
                        eprintln!("Error parsing JSON: {}", e);
                        panic!(); // add graceful error handling later
                    }
                }
            },
            Err(e) => {
                eprintln!("Error reading save data: {}", e);
                panic!(); // add graceful error handling later
            }
        }
    } else {
        println!("No save data found. Creating default calendar.");
        Calendar::create_default_calendar()
    }
}

fn write_to_json(calendar: &Calendar) -> std::io::Result<()> {
    let file = File::create(SAVE_DATA_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, calendar)?;
    Ok(())
}