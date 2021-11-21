use indexed_vec::newtype_index;
use pd_parse::{parse, Parse};
use pd_syntax::ast;
use std::sync::Arc;

newtype_index!(FileId);

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase: salsa::Database {
    #[salsa::input]
    fn input_text(&self, file_id: FileId) -> Arc<String>;
    fn parse_file(&self, file_id: FileId) -> Parse<ast::SourceFile>;
}

fn parse_file(db: &dyn SourceDatabase, file_id: FileId) -> Parse<ast::SourceFile> {
    parse::parse(&db.input_text(file_id))
}
