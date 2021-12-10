mod vfs;

pub use vfs::Vfs;

use indexvec::newtype_index;

newtype_index!(pub FileId);
