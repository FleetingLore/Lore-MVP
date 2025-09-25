// + [ * ]

pub struct DmnMeta {
    pub dmn: String,
}

impl DmnMeta {
    fn new() -> Self {
        DmnMeta {
            dmn: String::from("_dmn"),
        }
    }
}

use std::fmt::{Display, Formatter};

impl Display for DmnMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "+ {}", self.dmn)
    }
}
