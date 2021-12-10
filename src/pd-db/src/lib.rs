use salsa::{Database, Durability};

#[salsa::database(
    pd_base_db::SourceDatabaseStorage,
    pd_parse::AstDatabaseStorage,
    pd_ir::InternDatabaseStorage
)]
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
