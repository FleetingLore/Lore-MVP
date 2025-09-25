// - [ * ] = [ * ]

pub struct CatAtom {
    pub cat: String,
    pub value: String,
}

impl CatAtom {
    fn new() -> Self {
        CatAtom {
            cat: String::from("&dmn"),
            value: String::from("_val"),
        }
    }
}
