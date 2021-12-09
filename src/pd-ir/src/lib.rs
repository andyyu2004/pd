mod ir;
mod lower;
mod macros;

use salsa;
use std::sync::Arc;

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase {
    #[salsa::interned]
    fn intern_value_def(&self, value_def: Arc<ir::ValueDefData>) -> ir::ValueDef;
}

pub trait IrDatabase: InternDatabase {}
