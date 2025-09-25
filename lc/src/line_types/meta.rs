// [ * ]

pub struct Element {
    pub meta: String,
}

impl Element {
    fn new() -> Self {
        Element {
            meta: String::from("_meta"),
        }
    }
}

use std::fmt::{Display, Formatter};

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta)
    }
}
