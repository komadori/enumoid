#![doc = include_str!("../README.md")]

#[macro_use]
mod base;
mod iter;
mod map;
mod opt_map;
mod raw;
#[cfg(feature = "serde")]
mod serde;
mod set;
mod vec;

pub use base::EnumArrayHelper;
pub use base::Enumoid;
pub use base::Size;
pub use map::EnumMap;
pub use opt_map::EnumOptionMap;
pub use set::EnumSet;
pub use vec::EnumVec;

// Re-export derive macro
pub use enumoid_derive::*;
