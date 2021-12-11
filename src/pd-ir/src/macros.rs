#[macro_export]
macro_rules! intern_key {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name($crate::salsa::InternId);

        paste::paste! {
            impl $name {
                // Assumes the following conventions.
                // Suppose the intern key is called `Foo`, then:
                //  - the corresponding data is called `FooData`.
                //  - the interning method is called `intern_Foo`.
                pub fn lookup(self, db: &dyn $crate::InternDatabase) -> ::std::sync::Arc<[<$name Loc>]> {
                    db.[<lookup_intern_$name:snake>](self)
                }
            }
        }

        impl $crate::salsa::InternKey for $name {
            fn from_intern_id(v: $crate::salsa::InternId) -> Self {
                $name(v)
            }

            fn as_intern_id(&self) -> $crate::salsa::InternId {
                self.0
            }
        }
    };
}
