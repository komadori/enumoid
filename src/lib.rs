//! This crate is a Rust library which provides containers indexed by enums.

pub mod base;
mod flags;
mod iter;
mod map;
mod opt_map;
#[cfg(feature = "serde")]
mod serde;
mod vec;

pub use base::Enumoid;
pub use flags::EnumFlags;
pub use map::EnumMap;
pub use opt_map::EnumOptionMap;
pub use vec::EnumVec;
