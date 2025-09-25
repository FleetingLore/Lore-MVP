// - [ * ] # [ * ]

pub struct CatChain {
    pub cat: String,
    pub tag: String,
}

impl CatChain {
    fn new() -> Self {
        CatChain {
            cat: String::from("&dmn"),
            tag: String::from("&meta"),
        }
    }
}
