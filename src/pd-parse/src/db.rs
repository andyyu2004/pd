use pd_base_db::SourceDatabase;
use pd_syntax::ast;
use pd_vfs::FileId;

use crate::{parse, Parse};

#[salsa::query_group(AstDatabaseStorage)]
pub trait AstDatabase: SourceDatabase {
    fn parse_file(&self, file_id: FileId) -> Parse<ast::SourceFile>;
}

fn parse_file(db: &dyn AstDatabase, file_id: FileId) -> Parse<ast::SourceFile> {
    parse(&db.file_text(file_id))
}
