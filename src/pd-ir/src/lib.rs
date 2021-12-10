mod ir;
mod lower;
mod macros;

use pd_parse::AstDatabase;
use pd_vfs::FileId;
use salsa;
use std::sync::Arc;

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase {
    #[salsa::interned]
    fn intern_value_def(&self, value_def: Arc<ir::ValueDefData>) -> ir::ValueDef;
}

pub trait IrDatabase: InternDatabase + AstDatabase {
    fn file_items(&self, file_id: FileId) -> Arc<ir::Items>;
}

fn file_items(db: &dyn IrDatabase, file_id: FileId) -> Arc<ir::Items> {
    todo!()
}
