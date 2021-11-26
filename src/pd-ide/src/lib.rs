use std::sync::Arc;

pub use pd_db::FileId;

use pd_db::{RootDatabase, SourceDatabase};
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
        for (file_id, file_change) in change.files_changed {
            let new_text = match file_change {
                FileChange::Created(text) | FileChange::Modified(text) => text,
                FileChange::Deleted => String::new(),
            };
            self.db.set_file_text(file_id, Arc::new(new_text));
        }
    }
}

#[derive(Default)]
pub struct Change {
    files_changed: Vec<(FileId, FileChange)>,
}

impl Change {
    pub fn single(file_id: FileId, file_change: FileChange) -> Self {
        Self { files_changed: vec![(file_id, file_change)] }
    }

    pub fn change_file(&mut self, file_id: FileId, change: FileChange) {
        self.files_changed.push((file_id, change));
    }
}

pub enum FileChange {
    Created(String),
    Modified(String),
    Deleted,
}
