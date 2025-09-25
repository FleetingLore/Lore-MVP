// + [ * ] = [ * ]

pub struct DmnAtom {
    pub dmn: String,
    pub value: String,
}

impl DmnAtom {
    fn new() -> Self {
        DmnAtom {
            dmn: String::from("&dmn"),
            value: String::from("_val"),
        }
    }
}
