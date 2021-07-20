use crate::base::EnumArrayHelper;
use crate::base::EnumFlagsHelper;
use crate::base::Size;
use crate::iter::EnumSliceIter;
use crate::iter::EnumSliceIterMut;
use crate::opt_map::EnumOptionMap;
use num_traits::{AsPrimitive, Zero};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Debug;
use std::iter;
use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr;

/// A vector of values `V` indexed by enumoid `T`.
pub struct EnumVec<T: EnumArrayHelper<V>, V> {
  pub(crate) len: T::Word,
  pub(crate) data: T::PartialArray,
}

impl<T: EnumArrayHelper<V>, V> EnumVec<T, V> {
  pub fn new() -> Self {
    EnumVec {
      len: T::ZERO_WORD,
      data: T::new_partial(),
    }
  }

  pub fn new_with<F>(size: Option<Size<T>>, mut f: F) -> Self
  where
    F: FnMut(T) -> V,
  {
    let mut vec = Self::new();
    if let Some(sz) = size {
      for key in sz.iter() {
        vec.push(f(key));
      }
    }
    vec
  }

  pub fn as_slice(&self) -> &[V] {
    debug_assert!(
      self.len < T::SIZE_WORD,
      "Length out of bounds: {:?} >= {:?}",
      self.len,
      T::SIZE
    );
    unsafe {
      let inited =
        T::partial_slice(&self.data).get_unchecked(0..self.len.as_());
      &*(inited as *const [std::mem::MaybeUninit<V>] as *const [V])
    }
  }

  pub fn as_slice_mut(&mut self) -> &mut [V] {
    debug_assert!(
      self.len < T::SIZE_WORD,
      "Length out of bounds: {:?} >= {:?}",
      self.len,
      T::SIZE
    );
    unsafe {
      let inited = T::partial_slice_mut(&mut self.data)
        .get_unchecked_mut(0..self.len.as_());
      &mut *(inited as *mut [std::mem::MaybeUninit<V>] as *mut [V])
    }
  }

  pub fn get(&self, key: T) -> Option<&V> {
    let i = T::into_word(key);
    self.as_slice().get(i.as_())
  }

  pub fn is_empty(&self) -> bool {
    self.len.is_zero()
  }

  pub fn size(&self) -> Size<T> {
    unsafe { Size::<T>::from_word_unchecked(self.len) }
  }

  pub fn swap(&mut self, a: T, b: T) {
    self
      .as_slice_mut()
      .swap(T::into_word(a).as_(), T::into_word(b).as_())
  }

  pub fn clear(&mut self) {
    for cell in
      T::partial_slice_mut(&mut self.data)[0..self.len.as_()].iter_mut()
    {
      unsafe { ptr::drop_in_place(cell.as_mut_ptr()) };
    }
    self.len = T::ZERO_WORD;
  }

  pub fn push(&mut self, value: V) {
    let len = self.len.as_();
    T::partial_slice_mut(&mut self.data)[len] =
      mem::MaybeUninit::<V>::new(value);
    self.len = self.len + T::ONE_WORD;
  }

  #[inline]
  pub fn iter(&self) -> EnumSliceIter<T, V> {
    EnumSliceIter {
      _phantom: Default::default(),
      word: T::ZERO_WORD,
      iter: self.as_slice().iter(),
    }
  }

  #[inline]
  pub fn iter_mut(&mut self) -> EnumSliceIterMut<T, V> {
    EnumSliceIterMut {
      _phantom: Default::default(),
      word: T::ZERO_WORD,
      iter: self.as_slice_mut().iter_mut(),
    }
  }
}

impl<T: EnumArrayHelper<V> + Debug, V: Debug> Debug for EnumVec<T, V> {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt.debug_map().entries(self.iter()).finish()
  }
}

impl<T: EnumArrayHelper<V>, V> Default for EnumVec<T, V> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: EnumArrayHelper<V>, V> Drop for EnumVec<T, V> {
  fn drop(&mut self) {
    for cell in
      T::partial_slice_mut(&mut self.data)[0..self.len.as_()].iter_mut()
    {
      unsafe { ptr::drop_in_place(cell.as_mut_ptr()) };
    }
  }
}

impl<T: EnumArrayHelper<V>, V: Clone> Clone for EnumVec<T, V> {
  fn clone(&self) -> Self {
    let mut clone = Self::new();
    for (_, value) in self.iter() {
      clone.push(value.clone())
    }
    clone
  }
}

impl<T: EnumArrayHelper<V>, V> Index<T> for EnumVec<T, V> {
  type Output = V;

  fn index(&self, i: T) -> &V {
    &self.as_slice()[T::into_word(i).as_()]
  }
}

impl<T: EnumArrayHelper<V>, V> IndexMut<T> for EnumVec<T, V> {
  fn index_mut(&mut self, i: T) -> &mut V {
    &mut self.as_slice_mut()[T::into_word(i).as_()]
  }
}

impl<T: EnumArrayHelper<V>, V> iter::FromIterator<V> for EnumVec<T, V> {
  fn from_iter<I: iter::IntoIterator<Item = V>>(iter: I) -> Self {
    let mut c = EnumVec::<T, V>::new();
    for i in iter {
      c.push(i);
    }
    c
  }
}

impl<'a, T: EnumArrayHelper<V>, V> iter::IntoIterator for &'a EnumVec<T, V> {
  type Item = (T, &'a V);
  type IntoIter = EnumSliceIter<'a, T, V>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T: EnumFlagsHelper + EnumArrayHelper<V>, V>
  TryFrom<EnumOptionMap<T, V>> for EnumVec<T, V>
{
  type Error = ();
  fn try_from(from: EnumOptionMap<T, V>) -> Result<Self, Self::Error> {
    match from.is_vec() {
      Some(size) => Ok(EnumVec {
        len: size.size(),
        data: from.data,
      }),
      None => Err(()),
    }
  }
}
