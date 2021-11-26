use indexed_vec::Idx;
use pd_ide::FileId;
use std::path::PathBuf;

use indexmap::IndexSet;

#[derive(Debug, Default)]
pub(crate) struct Vfs {
    pub(crate) paths: IndexSet<PathBuf>,
}

impl Vfs {
    pub(crate) fn intern_path(&mut self, path: impl Into<PathBuf>) -> FileId {
        let path = path.into();
        assert!(path.is_absolute());
        let (idx, _) = self.paths.insert_full(path);
        FileId::new(idx)
    }
}
