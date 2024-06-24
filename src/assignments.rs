pub struct Assignment {
    name: String,
    num: i32,
}

impl Assignment {

    pub fn new(name: String, num: i32) -> Self {
        Assignment { name, num }
    }

    pub fn get_info(&self) -> (&str, i32) {
        (&self.name, self.num)
    }
}

// pub fn list_all_assignments() {
//     // something
// }