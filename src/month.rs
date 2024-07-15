use crate::day::CalDay;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CalMonth {
    pub name: String,
    pub days: Vec<CalDay>,
}