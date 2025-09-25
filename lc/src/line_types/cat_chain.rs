// - [ * ] # [ * ]

pub struct AtomChain {
    pub cat: String,
    pub tag: String,
}

impl AtomChain {
    fn new() -> Self {
        AtomChain {
            cat: String::from("&dmn"),
            tag: String::from("&meta"),
        }
    }
}
