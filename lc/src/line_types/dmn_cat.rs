// + [ * ] - [ * ]

pub struct DmnCat {
    pub cat: String,
    pub tag: String,
}

impl DmnCat {
    fn new() -> Self {
        DmnCat {
            cat: String::from("&dmn"),
            tag: String::from("&meta"),
        }
    }
}
