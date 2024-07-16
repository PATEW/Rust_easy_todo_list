use serde_derive::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Serialize, Deserialize)]
pub enum ItemType {
    Assignment,
    Meeting,
}

#[derive(Serialize, Deserialize)]
pub enum KanbanStatus {
    Backburner,
    Todo,
    InProgress,
    Done,
    None,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    event_type: ItemType,
    description: String,
    start: PrimitiveDateTime,
    end: PrimitiveDateTime,
    kanban_status: KanbanStatus,
}

impl Item {

    pub fn new(name: String, event_type: ItemType, description: String, start: PrimitiveDateTime, end: PrimitiveDateTime, kanban_status: KanbanStatus) -> Self {
        Item { name, event_type, description, start, end, kanban_status }
    }


}

// pub fn list_all_assignments() {
//     // something
// }