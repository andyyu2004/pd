mod expr;
mod pat;
mod ty;

pub use expr::*;
pub use pat::*;
pub use ty::*;

use la_arena::{Arena, Idx};
use pd_syntax::{ast, AstMethods};
use pd_vfs::FileId;
use smol_str::SmolStr;

use crate::intern_key;
use crate::resolve::Module;

intern_key!(Const);

pub type ConstLoc = ItemLoc<ConstData>;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ItemLoc<N> {
    pub container: Module,
    pub id: ItemId<N>,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ItemId<N> {
    file: FileId,
    /// Index into the items for the file
    value: Idx<N>,
}

impl<N> ItemId<N> {
    pub fn new(file: FileId, value: Idx<N>) -> Self {
        Self { file, value }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ConstData {
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
