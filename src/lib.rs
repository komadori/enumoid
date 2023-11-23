#![doc = include_str!("../README.md")]

#[macro_use]
mod base;
mod iter;
mod map;
mod opt_map;
#[cfg(feature = "serde")]
mod serde;
mod set;
mod sub_base;
mod vec;

pub use base::EnumArrayHelper;
pub use base::EnumIndex;
pub use base::EnumSetHelper;
pub use base::EnumSize;
pub use base::Enumoid;
pub use map::EnumMap;
pub use opt_map::EnumOptionMap;
pub use set::EnumSet;
pub use vec::EnumVec;

// Re-export derive macro
pub use enumoid_derive::*;
