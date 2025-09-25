// + [ * ] = [ * ]

pub struct DmnAtom {
    pub cat: String,
    pub tag: String,
}

impl DmnAtom {
    fn new() -> Self {
        crate::line_types::cat_chain::AtomChain {
            cat: String::from("&dmn"),
            tag: String::from("&meta"),
        }
    }
}
