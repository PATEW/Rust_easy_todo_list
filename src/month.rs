use crate::day::CalDay;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct CalMonth {
    pub name: String,
    pub days: Vec<CalDay>,
}