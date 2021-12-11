mod expr;
mod ty;

pub use expr::*;
pub use ty::*;

use pd_syntax::{ast, AstMethods};
use smol_str::SmolStr;

use crate::intern_key;

intern_key!(ValueDef);

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ValueDefData {
    pub name: Name,
    pub ty: Option<Type>,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Name(SmolStr);

impl From<ast::Ident> for Name {
    fn from(name: ast::Ident) -> Self {
        // TODO: is there way to do this without doing a double allocations?
        // smolstr only takes a str but we are giving it a string
        Self(SmolStr::new(name.syntax().first_child().unwrap().text().to_string()))
    }
}

pub type Items = Vec<Item>;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Item {
    ValueDef(ValueDef),
}

impl From<ValueDef> for Item {
    fn from(v: ValueDef) -> Self {
        Self::ValueDef(v)
    }
}
