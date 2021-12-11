use la_arena::{Arena, Idx};
use pd_syntax::ast;
use pd_syntax::{AstChildren, HasPat, HasType};

use crate::ir::*;
use crate::DefDatabase;

pub(crate) struct LowerCtxt<'db> {
    db: &'db dyn DefDatabase,
    data: ItemData,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Items {
    indexes: Vec<Item>,
    data: ItemData,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Item {
    Const(Idx<ConstData>),
}

impl From<Idx<ConstData>> for Item {
    fn from(v: Idx<ConstData>) -> Self {
        Self::Const(v)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct ItemData {
    consts: Arena<ConstData>,
}

impl<'db> LowerCtxt<'db> {
    pub(crate) fn new(db: &'db dyn DefDatabase) -> Self {
        Self { db, data: Default::default() }
    }

    pub(crate) fn lower_items(&mut self, ast: AstChildren<ast::Item>) -> Items {
        let mut items = Items::default();
        items.indexes = ast.flat_map(|item| self.lower_item(item)).collect();
        items
    }

    pub(crate) fn lower_item(&mut self, item: ast::Item) -> Option<Item> {
        let item = match item {
            ast::Item::Const(c) => self.lower_const(c)?.into(),
        };
        Some(item)
    }

    fn lower_const(&mut self, c: ast::Const) -> Option<Idx<ConstData>> {
        let pat = self.lower_pat(c.pat()?);
        let ty = c.ty().map(|ty| self.lower_ty(ty));
        let data = ConstData { pat, ty };
        Some(self.data.consts.alloc(data))
    }

    pub(crate) fn lower_ty(&self, ty: ast::Type) -> Type {
        match ty {
            ast::Type::Path(_) => todo!(),
        }
    }

    pub(crate) fn lower_pat(&self, pat: ast::Pat) -> Pat {
        match pat {
            ast::Pat::Binding(_) => todo!(),
            ast::Pat::Literal(lit) => Pat::Literal(),
        }
    }
}
