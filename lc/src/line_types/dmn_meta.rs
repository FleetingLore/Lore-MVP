// + [ * ]

pub struct DmnMeta {
    pub dmn: String,
}

impl DmnMeta {
    fn new() -> Self {
        DmnMeta {
            dmn: String::from("_dmn"),
        }
    }
}
