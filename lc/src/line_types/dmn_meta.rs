// + [ * ]

pub struct DmnMeta {
    pub cat: String,
    pub tag: String,
}

impl DmnMeta {
    fn new() -> Self {
        DmnMeta {
            cat: String::from("&dmn"),
            tag: String::from("&meta"),
        }
    }
}
