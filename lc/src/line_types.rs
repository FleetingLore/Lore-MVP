pub mod atom_meta;
pub use atom_meta::AtomMeta;

pub mod anchor_chain;
pub use anchor_chain::AnchorChain;

pub mod anchor_meta;
pub use anchor_meta::AnchorMeta;

pub mod cat_meta;
pub use cat_meta::CatMeta;

pub mod cat_atom;
pub use cat_atom::CatAtom;

pub mod cat_chain;
pub use cat_chain::CatChain;

pub mod dmn_meta;
pub use dmn_meta::DmnMeta;

pub mod dmn_atom;
pub use dmn_atom::DmnAtom;

pub mod dmn_chain;
pub use dmn_chain::DmnChain;

pub mod dmn_cat;
pub use dmn_cat::DmnCat;

pub mod element;
pub use element::Element;

pub enum AtomLine {
    FromMeta  (AtomMeta)
}

pub enum AnchorLine {
    FromChain (AnchorChain),
    FromMeta  (AnchorMeta)
}

pub enum CatLine {
    FromAtom  (CatAtom),
    FromChain (CatChain),
    FromMeta  (CatMeta)
}

pub enum DmnLine {
    FromAtom  (DmnAtom),
    FromChain (DmnChain),
    FromCat   (DmnCat),
    FromMeta  (DmnMeta)
}

pub enum Line {
    Atom      (AtomLine),
    Anchor    (AnchorLine),
    Category  (CatLine),
    Domain    (DmnLine),
    Element   (Element)
}
