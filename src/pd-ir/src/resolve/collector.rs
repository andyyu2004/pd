use std::ops::Deref;

use indexvec::Idx;
use pd_vfs::FileId;

use crate::lower::Item;
use crate::DefDatabase;

use super::{Defs, Module, ModuleData};

pub(super) struct DefCollector<'db> {
    db: &'db dyn DefDatabase,
    defs: Defs,
}

impl<'db> DefCollector<'db> {
    pub(super) fn new(db: &'db dyn DefDatabase) -> Self {
        Self { db, defs: Defs::default() }
    }

    pub(crate) fn finish(self) -> Defs {
        self.defs
    }

    pub(crate) fn collect(&mut self) {
        let root_module = self.defs.modules.alloc(ModuleData { items: Default::default() });
        ModCollector::new(self, root_module).collect()
    }
}

pub(super) struct ModCollector<'a, 'db> {
    def_collector: &'a mut DefCollector<'db>,
    module: Module,
}

impl<'a, 'db> ModCollector<'a, 'db> {
    pub(super) fn new(def_collector: &'a mut DefCollector<'db>, module: Module) -> Self {
        Self { def_collector, module }
    }

    pub(super) fn collect(&mut self) {
        let file = FileId::new(0);
        let items = self.db.file_items(file);
        for item in items.as_ref() {
            match item {
                Item::Const(konst) => {
                    // let c = &items[konst];
                    // let k = ConstLoc { container: self.module, id: ItemId::new(file, konst.idx) };
                    // let konst = self.db.intern_const(k);
                    todo!()
                }
            }
        }
    }
}

impl<'db> Deref for ModCollector<'_, 'db> {
    type Target = DefCollector<'db>;

    fn deref(&self) -> &Self::Target {
        self.def_collector
    }
}
