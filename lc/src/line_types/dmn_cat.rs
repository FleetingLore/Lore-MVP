// + [ * ] - [ * ]

pub struct DmnCat {
    pub dmn: String,
    pub cat: String,
}

impl DmnCat {
    fn new() -> Self {
        DmnCat {
            dmn: String::from("_dmn"),
            cat: String::from("&dmn"),
        }
    }
}

use std::fmt::{Display, Formatter};

impl Display for DmnCat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "+ {} - {}", self.dmn, self.cat)
    }
}
