use serde_derive::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct Assignment {
    name: String,
    description: String,
    start: PrimitiveDateTime,
    end: PrimitiveDateTime,
    // Kanban_status: status enum?
}

impl Assignment {

    pub fn new(name: String, description: String, start: PrimitiveDateTime, end: PrimitiveDateTime) -> Self {
        Assignment { name, description, start, end }
    }


}

// pub fn list_all_assignments() {
//     // something
// }