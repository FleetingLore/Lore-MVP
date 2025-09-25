// + [ * ] = [ * ]

pub struct DmnAtom {
    pub dmn: String,
    pub value: String,
}

impl DmnAtom {
    fn new() -> Self {
        DmnAtom {
            dmn: String::from("&dmn"),
            value: String::from("_val"),
        }
    }
}

use std::fmt::{Display, Formatter};

impl Display for DmnAtom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "+ {} = {}", self.dmn, self.value)
    }
}
