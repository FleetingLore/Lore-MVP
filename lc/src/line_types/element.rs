// [ * ]

pub struct Element {
    pub element: String,
}

impl Element {
    fn new() -> Self {
        Element {
            element: String::from("_meta"),
        }
    }
}
