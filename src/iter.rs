use crate::base::Enumoid;
use std::iter;
use std::marker;
use std::slice;

pub struct EnumSliceIter<'a, T, V: 'a> {
  pub(crate) _phantom: marker::PhantomData<T>,
  pub(crate) iter: iter::Enumerate<slice::Iter<'a, V>>,
}

impl<'a, T: Enumoid, V> EnumSliceIter<'a, T, V> {
  #[inline(always)]
  fn f(iv: (usize, &'a V)) -> (T, &'a V) {
    (T::from_usize(iv.0), iv.1)
  }
}

impl<'a, T: Enumoid, V> Iterator for EnumSliceIter<'a, T, V> {
  type Item = (T, &'a V);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(Self::f)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }

  #[inline]
  fn fold<B, F>(self, init: B, f: F) -> B
  where
    F: FnMut(B, Self::Item) -> B,
  {
    self.iter.map(Self::f).fold(init, f)
  }
}

pub struct EnumSliceIterMut<'a, T, V: 'a> {
  pub(crate) _phantom: marker::PhantomData<T>,
  pub(crate) iter: iter::Enumerate<slice::IterMut<'a, V>>,
}

impl<'a, T: Enumoid, V> EnumSliceIterMut<'a, T, V> {
  #[inline(always)]
  fn f(iv: (usize, &'a mut V)) -> (T, &'a mut V) {
    (T::from_usize(iv.0), iv.1)
  }
}

impl<'a, T: Enumoid, V> Iterator for EnumSliceIterMut<'a, T, V> {
  type Item = (T, &'a mut V);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(Self::f)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }

  #[inline]
  fn fold<B, F>(self, init: B, f: F) -> B
  where
    F: FnMut(B, Self::Item) -> B,
  {
    self.iter.map(Self::f).fold(init, f)
  }
}
