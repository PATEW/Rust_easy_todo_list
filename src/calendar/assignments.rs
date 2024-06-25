use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Assignment {
    name: String,
    description: String,
    due: String,
}

impl Assignment {

    pub fn new(name: String, description: String, due: String) -> Self {
        Assignment { name, description, due }
    }

    pub fn get_info(&self) -> (&str, &str, &str) {
        (&self.name, &self.description, &self.due)
    }
}

// pub fn list_all_assignments() {
//     // something
// }