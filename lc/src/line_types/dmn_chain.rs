// + [ * ] # [ * ]

pub struct DmnChain {
    pub dmn: String,
    pub tag: String,
}

impl DmnChain {
    fn new() -> Self {
        DmnChain {
            dmn: String::from("_dmn"),
            tag: String::from("&meta"),
        }
    }
}
