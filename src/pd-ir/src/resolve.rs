use la_arena::{Arena, Idx};
use rustc_hash::FxHashMap;

use crate::ir::{Items, Name, ValueDef};

#[derive(Debug, PartialEq, Eq)]
pub struct Defs {
    pkg: Pkg,
    modules: Arena<ModuleData>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ModuleData {
    pub items: FxHashMap<Name, Module>,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum ModuleDefId {
    ValueDef(ValueDef),
}

pub type Pkg = Idx<PkgData>;
pub type Module = Idx<ModuleData>;

pub struct PkgData;
