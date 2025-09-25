// + [ * ] - [ * ]

pub struct DmnCat {
    pub dmn: String,
    pub cat: String,
}

impl DmnCat {
    fn new() -> Self {
        DmnCat {
            dmn: String::from("_dmn"),
            cat: String::from("&dmn"),
        }
    }
}
