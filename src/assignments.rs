use serde_derive::Serialize;
use time::PrimitiveDateTime;

#[derive(Serialize)]
pub struct Assignment {
    name: String,
    description: String,
    start: (String, String),
    end: (String, String),
}

impl Assignment {

    pub fn new(name: String, description: String, start: (String, String), end: (String, String)) -> Self {
        Assignment { name, description, start, end }
    }



    // pub fn get_info(&self) -> (&str, &str, &str) {
    //     (&self.name, &self.description, &self.due)
    // }
}

// pub fn list_all_assignments() {
//     // something
// }