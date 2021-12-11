use la_arena::{Arena, Idx};
use pd_syntax::ast;
use pd_syntax::{AstChildren, AstMethods, HasPat, HasType};
use smol_str::SmolStr;

use crate::ir::*;
use crate::DefDatabase;

pub(crate) struct LowerCtxt<'db> {
    db: &'db dyn DefDatabase,
    data: ItemData,
}

#[derive(Debug, Default, Eq, PartialEq)]
struct ItemData {
    value_defs: Arena<ValueDefData>,
}

impl<'db> LowerCtxt<'db> {
    pub(crate) fn new(db: &'db dyn DefDatabase) -> Self {
        Self { db, data: Default::default() }
    }

    pub(crate) fn lower_items(&mut self, items: AstChildren<ast::Item>) -> Items {
        items.flat_map(|item| self.lower_item(item)).collect()
    }

    pub(crate) fn lower_item(&mut self, item: ast::Item) -> Option<Item> {
        let item = match item {
            ast::Item::ValueDef(value_def) => self.lower_value_def(value_def)?.into(),
        };
        Some(item)
    }

    fn lower_value_def(&mut self, value_def: ast::ValueDef) -> Option<Idx<ValueDefData>> {
        let pat = self.lower_pat(value_def.pat()?);
        let ty = value_def.ty().map(|ty| self.lower_ty(ty));
        let value_def = ValueDefData { pat, ty };
        Some(self.data.value_defs.alloc(value_def))
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
