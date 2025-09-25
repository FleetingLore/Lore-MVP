// - [ * ] = [ * ]

pub struct CatAtom {
    pub cat: String,
    pub value: String,
}

impl CatAtom {
    fn new() -> Self {
        CatAtom {
            cat: String::from("&dmn"),
            value: String::from("_val"),
        }
    }
}

use std::fmt::{Display, Formatter};

impl Display for CatAtom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "+ {} = {}", self.cat, self.value)
    }
}
