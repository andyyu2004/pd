use pd_vfs::FileId;
use std::sync::Arc;

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase: salsa::Database {
    #[salsa::input]
    fn file_text(&self, file_id: FileId) -> Arc<String>;
}
