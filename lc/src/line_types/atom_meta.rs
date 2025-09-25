// [ * ] = [ * ]

pub struct AtomMeta {
    pub element: String,
    pub value: String,
}

impl AtomMeta {
    fn new() -> Self {
        AtomMeta {
            element: String::from("_meta"),
            value: String::from("_val"),
        }
    }
}
