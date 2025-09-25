// + [ * ] # [ * ]

pub struct DmnChain {
    pub cat: String,
    pub tag: String,
}

impl DmnChain {
    fn new() -> Self {
        DmnChain {
            cat: String::from("&dmn"),
            tag: String::from("&meta"),
        }
    }
}
