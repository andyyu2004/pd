use pd_syntax::ast;
use pd_syntax::{AstChildren, AstMethods, HasName, HasType};
use smol_str::SmolStr;

use crate::ir::*;
use crate::DefDatabase;

pub(crate) struct LowerCtxt<'db> {
    db: &'db dyn DefDatabase,
}

impl<'db> LowerCtxt<'db> {
    pub(crate) fn new(db: &'db dyn DefDatabase) -> Self {
        Self { db }
    }

    pub(crate) fn lower_items(&self, items: AstChildren<ast::Item>) -> Items {
        items.flat_map(|item| self.lower_item(item)).collect()
    }

    pub(crate) fn lower_item(&self, item: ast::Item) -> Option<Item> {
        let item = match item {
            ast::Item::ValueDef(value_def) => self.lower_value_def(value_def)?.into(),
        };
        Some(item)
    }

    fn lower_value_def(&self, value_def: ast::ValueDef) -> Option<ValueDef> {
        dbg!(&value_def);
        let name = Name::from(value_def.name()?);
        let ty = value_def.ty().map(|ty| self.lower_ty(ty));
        let value_def = ValueDefData { name, ty };
        todo!()
    }

    pub(crate) fn lower_ty(&self, ty: ast::Type) -> Type {
        match ty {
            ast::Type::Path(_) => todo!(),
        }
    }
}
