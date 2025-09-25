use std::fmt::{Display, Formatter};
use crate::line_types::*;
use Line::*;

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom     ( AsAtom   ) => AsAtom   .fmt(f),
            Anchor   ( AsAnchor ) => AsAnchor .fmt(f),
            Category ( AsCat    ) => AsCat    .fmt(f),
            Domain   ( AsDmn    ) => AsDmn    .fmt(f),
            Element  ( element  ) => write!(f, "{}", element.meta)
        }
    }
}

impl Display for AsAtom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AsAtom::FromMeta( atom_meta ) => atom_meta .fmt(f),
        }
    }
}

impl Display for AsAnchor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AsAnchor::FromMeta( anchor_meta ) => anchor_meta .fmt(f),
            AsAnchor::FromDrop( anchor_drop ) => anchor_drop .fmt(f),
        }
    }
}

impl Display for AsCat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AsCat::FromMeta(  cat_meta  ) => cat_meta  .fmt(f),
            AsCat::FromDrop( cat_chain  ) => cat_chain .fmt(f),
            AsCat::FromAtom(  cat_atom  ) => cat_atom  .fmt(f),
        }
    }
}

impl Display for AsDmn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AsDmn::FromMeta( dmn_meta  ) => dmn_meta  .fmt(f),
            AsDmn::FromDrop( dmn_chain ) => dmn_chain .fmt(f),
            AsDmn::FromAtom( dmn_atom  ) => dmn_atom  .fmt(f),
            AsDmn::FromCat(  dmn_cat   ) => dmn_cat   .fmt(f),
        }
    }
}
