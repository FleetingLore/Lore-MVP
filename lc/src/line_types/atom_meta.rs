// [ * ] = [ * ]

pub struct AtomMeta {
    pub element: String,
    pub value: String,
}

impl AtomMeta {
    fn new() -> Self {
        AtomMeta {
            element: String::from("_meta"),
            value: String::from("_val"),
        }
    }
}

use std::fmt::{Display, Formatter};
impl Display for AtomMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.element, self.value)
    }
}
