use std::cell::Cell;

/// A value that records its own destruction in a shared counter.
///
/// The consuming `IntoIterator` implementations move elements out of
/// `MaybeUninit`-backed storage with unsafe code, so these tests use the
/// counter to assert that every element is either yielded or dropped exactly
/// once, with no leaks and no double frees.
pub struct DropTracker<'a> {
  id: i32,
  drops: &'a Cell<u32>,
}

impl<'a> DropTracker<'a> {
  pub fn new(id: i32, drops: &'a Cell<u32>) -> Self {
    DropTracker { id, drops }
  }

  pub fn id(&self) -> i32 {
    self.id
  }
}

impl Drop for DropTracker<'_> {
  fn drop(&mut self) {
    self.drops.set(self.drops.get() + 1);
  }
}
