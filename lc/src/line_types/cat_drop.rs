// - [ * ] # [ * ]

pub struct CatDrop {
    pub cat: String,
    pub tag: String,
}

impl CatDrop {
    fn new() -> Self {
        CatDrop {
            cat: String::from("&dmn"),
            tag: String::from("&meta"),
        }
    }
}

use std::fmt::{Display, Formatter};

impl Display for CatDrop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "+ {} # {}", self.cat, self.tag)
    }
}
