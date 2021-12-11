use crate::DefDatabase;
use indexvec::Idx;
use pd_base_db::SourceDatabase;
use pd_vfs::FileId;
use std::sync::Arc;

#[salsa::database(
    pd_base_db::SourceDatabaseStorage,
    pd_parse::AstDatabaseStorage,
    crate::InternDatabaseStorage,
    crate::DefDatabaseStorage
)]
#[derive(Default)]
pub(crate) struct TestDb {
    storage: salsa::Storage<TestDb>,
}

impl salsa::Database for TestDb {
}

#[test]
fn test_lower_items() {
    let mut db = TestDb::default();
    let file_id = FileId::new(0);
    db.set_file_text(file_id, Arc::new("let x = false".to_string()));
    let items = db.file_items(file_id);
}
