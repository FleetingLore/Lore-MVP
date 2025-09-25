// - [ * ]

pub struct CatMeta {
    pub cat: String,
}

impl CatMeta {
    fn new() -> Self {
        CatMeta {
            cat: String::from("&dmn"),
        }
    }
}

use std::fmt::{Display, Formatter};

impl Display for CatMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {}", self.cat)
    }
}
