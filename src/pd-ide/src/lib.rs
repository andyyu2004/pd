use pd_db::{FileId, RootDatabase};
use salsa::ParallelDatabase;

pub struct Analysis {
    snapshot: salsa::Snapshot<RootDatabase>,
}

#[derive(Default)]
pub struct AnalysisCtxt {
    db: RootDatabase,
}

impl AnalysisCtxt {
    pub fn new() -> Self {
        Self { db: Default::default() }
    }

    pub fn analysis(&self) -> Analysis {
        Analysis { snapshot: self.db.snapshot() }
    }

    pub fn apply_change(&mut self, change: Change) {
    }
}

#[derive(Default)]
pub struct Change {
    files_changed: Vec<(FileId, FileChange)>,
}

impl Change {
    pub fn change_file(&mut self, file_id: FileId, change: FileChange) {
        self.files_changed.push((file_id, change));
    }
}

pub enum FileChange {
    Created,
    Modified(String),
    Deleted,
}
