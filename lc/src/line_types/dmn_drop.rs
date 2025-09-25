// + [ * ] # [ * ]

pub struct DmnDrop {
    pub dmn: String,
    pub tag: String,
}

impl DmnDrop {
    fn new() -> Self {
        DmnDrop {
            dmn: String::from("_dmn"),
            tag: String::from("&meta"),
        }
    }
}

use std::fmt::{Display, Formatter};

impl Display for DmnDrop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "+ {} # {}", self.dmn, self.tag)
    }
}

