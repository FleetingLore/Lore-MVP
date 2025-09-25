pub mod atom_meta;
pub mod anchor_drop;
pub mod anchor_meta;
pub mod cat_meta;
pub mod cat_atom;
pub mod cat_drop;
pub mod dmn_meta;
pub mod dmn_atom;
pub mod dmn_drop;
pub mod dmn_cat;
pub mod meta;
pub use atom_meta::AtomMeta;
pub use anchor_drop::AnchorDrop;
pub use anchor_meta::AnchorMeta;
pub use cat_meta::CatMeta;
pub use cat_atom::CatAtom;
pub use cat_drop::CatDrop;
pub use dmn_meta::DmnMeta;
pub use dmn_atom::DmnAtom;
pub use dmn_cat::DmnCat;
pub use dmn_drop::DmnDrop;
pub use meta::Element;

pub enum AsAtom {
    FromMeta  (AtomMeta)
}
pub enum AsAnchor {
    FromDrop(AnchorDrop),
    FromMeta  (AnchorMeta)
}
pub enum AsCat {
    FromAtom  (CatAtom),
    FromDrop(CatDrop),
    FromMeta  (CatMeta)
}
pub enum AsDmn {
    FromAtom  (DmnAtom),
    FromDrop(DmnDrop),
    FromCat   (DmnCat),
    FromMeta  (DmnMeta)
}

pub enum Line {
    Atom      (AsAtom),
    Anchor    (AsAnchor),
    Category  (AsCat),
    Domain    (AsDmn),
    Element   (Element)
}
impl Line {
    fn display(&self, indent: usize) -> String {
        format!("{indent}{self}")
    }
}
