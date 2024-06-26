use crate::assignments::Assignment;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct CalDay {
    pub date: String,
    pub assignments: Vec<Assignment>,
}