// [ * ] # [ * ]

pub struct AnchorDrop {
    pub element: String,
    pub tag: String,
}

impl AnchorDrop {
    pub fn new() -> Self {
        AnchorDrop {
            element: String::from("_meta"),
            tag: String::from("&meta"),
        }
    }
}

use std::fmt::{Display, Formatter};

impl Display for AnchorDrop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} # {}", self.element, self.tag)
    }
}
