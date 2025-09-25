// # [ * ]

pub struct AnchorMeta {
    pub tag: String,
}

impl AnchorMeta {
    pub fn new() -> Self {
        AnchorMeta {
            tag: String::from("&meta"),
        }
    }
}

use std::fmt::{Display, Formatter};

impl Display for AnchorMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "# {}",self.tag)
    }
}
