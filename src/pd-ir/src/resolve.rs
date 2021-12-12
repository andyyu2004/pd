mod collector;

use la_arena::{Arena, Idx};
use rustc_hash::FxHashMap;

use crate::ir::{Const, Name};
use crate::DefDatabase;

use self::collector::DefCollector;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Defs {
    modules: Arena<ModuleData>,
}

impl Defs {
    pub(crate) fn collect(db: &dyn DefDatabase) -> Defs {
        let mut collector = DefCollector::new(db);
        collector.collect();
        collector.finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ModuleData {
    pub items: FxHashMap<Name, Module>,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum ModuleDefId {
    Const(Const),
}

pub type Pkg = Idx<PkgData>;
pub type Module = Idx<ModuleData>;

pub struct PkgData;
