// [ * ]

pub struct Element {
    pub cat: String,
    pub tag: String,
}

impl Element {
    fn new() -> Self {
        Element {
            cat: String::from("&dmn"),
            tag: String::from("&meta"),
        }
    }
}
