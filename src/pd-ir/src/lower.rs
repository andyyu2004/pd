use std::hash::Hash;
use std::ops::Index;

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

impl<'a> IntoIterator for &'a Items {
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, Item>>;
    type Item = Item;

    fn into_iter(self) -> Self::IntoIter {
        self.indexes.iter().copied()
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Const(ItemIdx<ConstData>),
}

impl Index<ItemIdx<ConstData>> for Items {
    type Output = ConstData;

    fn index(&self, index: ItemIdx<ConstData>) -> &Self::Output {
        &self.data.consts[index.idx]
    }
}

#[derive(Debug)]
pub struct ItemIdx<N> {
    idx: Idx<N>,
}

impl<N> Eq for ItemIdx<N> {
}

impl<N> PartialEq for ItemIdx<N> {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl<N> Hash for ItemIdx<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}

impl<N> Clone for ItemIdx<N> {
    fn clone(&self) -> Self {
        Self { idx: self.idx.clone() }
    }
}

impl<N> Copy for ItemIdx<N> {
}

impl From<ItemIdx<ConstData>> for Item {
    fn from(v: ItemIdx<ConstData>) -> Self {
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

    fn lower_const(&mut self, c: ast::Const) -> Option<ItemIdx<ConstData>> {
        let pat = self.lower_pat(c.pat()?);
        let ty = c.ty().map(|ty| self.lower_ty(ty));
        let data = ConstData { pat, ty };
        Some(id(self.data.consts.alloc(data)))
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

fn id(idx: Idx<ConstData>) -> ItemIdx<ConstData> {
    ItemIdx { idx }
}
