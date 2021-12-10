use pd_parse::{parse, Parse};
use pd_syntax::ast;
use pd_vfs::FileId;
use salsa::{Database, Durability};
use std::sync::Arc;

#[salsa::database(SourceDatabaseStorage, pd_ir::InternDatabaseStorage)]
#[derive(Default)]
pub struct RootDatabase {
    storage: salsa::Storage<Self>,
}

impl RootDatabase {
    pub fn request_cancellation(&mut self) {
        self.salsa_runtime_mut().synthetic_write(Durability::LOW);
    }
}

impl salsa::Database for RootDatabase {
}

impl salsa::ParallelDatabase for RootDatabase {
    fn snapshot(&self) -> salsa::Snapshot<RootDatabase> {
        salsa::Snapshot::new(RootDatabase { storage: self.storage.snapshot() })
    }
}

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase: salsa::Database {
    #[salsa::input]
    fn file_text(&self, file_id: FileId) -> Arc<String>;
    fn parse_file(&self, file_id: FileId) -> Parse<ast::SourceFile>;
}

fn parse_file(db: &dyn SourceDatabase, file_id: FileId) -> Parse<ast::SourceFile> {
    parse(&db.file_text(file_id))
}
