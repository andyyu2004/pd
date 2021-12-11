mod expr;
mod pat;
mod ty;

pub use expr::*;
use la_arena::Idx;
pub use pat::*;
pub use ty::*;

use pd_syntax::{ast, AstMethods};
use smol_str::SmolStr;

use crate::intern_key;

intern_key!(ValueDef);

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ValueDefData {
    pub pat: Pat,
    pub ty: Option<Type>,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Name(SmolStr);

impl From<ast::Name> for Name {
    fn from(name: ast::Name) -> Self {
        // TODO: is there way to do this without doing a double allocations?
        // smolstr only takes a str but we are giving it a string
        Self(SmolStr::new(name.syntax().first_child().unwrap().text().to_string()))
    }
}

pub type Items = Vec<Item>;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Item {
    ValueDef(Idx<ValueDefData>),
}

impl From<Idx<ValueDefData>> for Item {
    fn from(v: Idx<ValueDefData>) -> Self {
        Self::ValueDef(v)
    }
}
