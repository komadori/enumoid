use crate::base::EnumArrayHelper;
use crate::base::Size;
use crate::flags::EnumFlags;
use num_traits::AsPrimitive;
use std::mem;
use std::ptr;

/// A partial map from enumoid `T` to values `V`.
pub struct EnumOptionMap<T: EnumArrayHelper<V>, V> {
  valid: EnumFlags<T>,
  pub(crate) data: T::PartialArray,
}

impl<T: EnumArrayHelper<V>, V> EnumOptionMap<T, V> {
  /// Creates a new, empty `EnumOptionMap<T, V>`.
  pub fn new() -> Self {
    EnumOptionMap {
      valid: EnumFlags::<T>::new(),
      data: T::new_partial(),
    }
  }

  pub fn get(&self, key: T) -> Option<&V> {
    let i = T::into_word(key);
    if self.valid.get_internal(i) {
      Some(unsafe { &*T::partial_slice(&self.data)[i.as_()].as_ptr() })
    } else {
      None
    }
  }

  pub fn set(&mut self, key: T, value: Option<V>) {
    let i = T::into_word(key);
    let cell = &mut T::partial_slice_mut(&mut self.data)[i.as_()];
    if self.valid.get_internal(i) {
      unsafe { ptr::drop_in_place(cell.as_mut_ptr()) };
    }
    self.valid.set_internal(i, value.is_some());
    if let Some(v) = value {
      *cell = mem::MaybeUninit::new(v);
    }
  }

  pub fn is_empty(&self) -> bool {
    !self.valid.any()
  }

  pub fn is_full(&self) -> bool {
    self.valid.all()
  }

  pub fn is_vec(&self) -> Option<Size<T>> {
    let mut seen_none = false;
    let mut size = T::ZERO_WORD;
    for (k, v) in self.valid.iter() {
      if v {
        if seen_none {
          return None;
        }
        size = T::into_word(k) + T::ONE_WORD;
      } else {
        seen_none = true;
      }
    }
    Some(unsafe { Size::<T>::from_word_unchecked(size) })
  }
}

impl<T: EnumArrayHelper<V>, V> Default for EnumOptionMap<T, V> {
  fn default() -> Self {
    EnumOptionMap::<T, V>::new()
  }
}
