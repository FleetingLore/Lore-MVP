// [ * ] # [ * ]

pub struct AnchorChain {
    pub element: String,
    pub tag: String,
}

impl AnchorChain {
    fn new() -> Self {
        AnchorChain {
            element: String::from("_meta"),
            tag: String::from("&meta"),
        }
    }
}
