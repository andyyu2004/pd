pub mod ir;

mod lower;
mod macros;
mod resolve;

use ir::*;
use pd_parse::{AstDatabase, Parse};
use pd_syntax::{ast, AstNodeExt, SyntaxNodeExt};
use pd_vfs::FileId;
use salsa;
use std::sync::Arc;

use crate::lower::LowerCtxt;

use self::lower::Items;
use self::resolve::Defs;

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase {
    #[salsa::interned]
    fn intern_const(&self, c: Arc<ConstLoc>) -> Const;
}

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: InternDatabase + AstDatabase {
    fn defs(&self) -> Arc<Defs>;
    fn file_items(&self, file_id: FileId) -> Arc<Items>;
    fn const_data(&self, c: Const) -> Arc<ConstData>;
}

fn file_items(db: &dyn DefDatabase, file_id: FileId) -> Arc<Items> {
    let source_file: Parse<ast::SourceFile> = db.parse_file(file_id);
    let node = source_file.syntax();
    let source_file = node.cast::<ast::SourceFile>().expect("expected SourceFile");
    let items = source_file.find_children::<ast::Item>();

    let mut lcx = LowerCtxt::new(db);
    let items = lcx.lower_items(items);
    Arc::new(items)
}

fn defs(db: &dyn DefDatabase) -> Arc<Defs> {
    // let mut defs = Defs::new();
    todo!()
}

fn const_data(db: &dyn DefDatabase, c: Const) -> Arc<ConstData> {
    todo!()
}

#[cfg(test)]
mod tests;
