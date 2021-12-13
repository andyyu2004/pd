use pd_ir::DefDatabase;

#[salsa::query_group(CodegenDatabaseStorage)]
pub trait CodegenDatabase: DefDatabase {}
