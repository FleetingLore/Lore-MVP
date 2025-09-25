pub enum Line {
    Element(String),
    Anchor(String, String),
    Atom(String),
    Chain(String, String),
    Cat(String),
    CatAnchor(String, String),
    CatAtom(String, String),
    Vein(String),
    VeinAnchor(String, String),
    VeinAtom(String, String),
    VeinCat(String, String),
    Omission
}

impl Line {
    fn display(self, indent: usize) -> String {
        let indent = " ".repeat(indent * 2);
        match self {
            Line::Element(t1)            => format!("{indent}{t1}", ),
            Line::Anchor(t1, t2)         => format!("{indent}{t1} = {t2}"),
            Line::Atom(t1)               => format!("{indent}# {t1}"),
            Line::Chain(t1, t2)          => format!("{indent}{t1} # {t2}"),
            Line::Cat(t1)                => format!("{indent}- {t1}"),
            Line::CatAnchor(t1, t2)  => format!("{indent}- {t1} = {t2}"),
            Line::CatAtom(t1, t2)    => format!("{indent}- {t1} # {t2}"),
            Line::Vein(t1)               => format!("{indent}+ {t1}"),
            Line::VeinAnchor(t1, t2) => format!("{indent}+ {t1} = {t2}"),
            Line::VeinAtom(t1, t2)   => format!("{indent}+ {t1} # {t2}"),
            Line::VeinCat(t1, t2)    => format!("{indent}+ {t1} - {t2}"),
            Line::Omission => String::new()
        }
    }
}
