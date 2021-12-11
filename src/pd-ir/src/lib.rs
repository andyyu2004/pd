mod ir;
mod lower;
mod macros;

use ir::*;
use pd_parse::{AstDatabase, Parse};
use pd_syntax::{ast, AstNodeExt, SyntaxNodeExt};
use pd_vfs::FileId;
use salsa;
use std::sync::Arc;

use crate::lower::LowerCtxt;

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase {
    #[salsa::interned]
    fn intern_value_def(&self, value_def: Arc<ValueDefData>) -> ValueDef;
}

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: InternDatabase + AstDatabase {
    fn file_items(&self, file_id: FileId) -> Arc<Items>;
}

fn file_items(db: &dyn DefDatabase, file_id: FileId) -> Arc<Items> {
    let source_file: Parse<ast::SourceFile> = db.parse_file(file_id);
    let node = source_file.syntax();
    let source_file = node.cast::<ast::SourceFile>().expect("expected SourceFile");
    let items = source_file.find_children::<ast::Item>();

    let lcx = LowerCtxt::new(db);
    let items = lcx.lower_items(items);
    dbg!(&items);
    todo!()
}

#[cfg(test)]
mod tests;
