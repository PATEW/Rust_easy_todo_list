use crate::assignments::Assignment;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CalDay {
    pub date: String,
    pub assignments: Vec<Assignment>,
}