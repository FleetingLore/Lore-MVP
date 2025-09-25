// # [ * ]

pub struct AnchorMeta {
    pub tag: String,
}

impl AnchorMeta {
    fn new() -> Self {
        AnchorMeta {
            tag: String::from("&meta"),
        }
    }
}
