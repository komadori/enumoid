use crate::base::Enumoid;
use crate::sub_base::RawSizeWord;
use std::marker;
use std::slice;

pub struct EnumSliceIter<'a, T: Enumoid, V: 'a> {
  pub(crate) _phantom: marker::PhantomData<T>,
  pub(crate) word: T::Word,
  pub(crate) iter: slice::Iter<'a, V>,
}

impl<'a, T: Enumoid, V> Iterator for EnumSliceIter<'a, T, V> {
  type Item = (T, &'a V);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let value = self.iter.next()?;
    let key = unsafe { T::from_word_unchecked(self.word) };
    self.word = self.word.inc();
    Some((key, value))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}

impl<'a, T: Enumoid, V> ExactSizeIterator for EnumSliceIter<'a, T, V> {}

impl<'a, T: Enumoid, V> DoubleEndedIterator for EnumSliceIter<'a, T, V> {
  #[inline]
  fn next_back(&mut self) -> Option<Self::Item> {
    let value = self.iter.next_back()?;
    let idx = self.word.as_() + self.iter.len();
    let key =
      unsafe { T::from_word_unchecked(T::Word::from_usize_unchecked(idx)) };
    Some((key, value))
  }
}

pub struct EnumSliceIterMut<'a, T: Enumoid, V: 'a> {
  pub(crate) _phantom: marker::PhantomData<T>,
  pub(crate) word: T::Word,
  pub(crate) iter: slice::IterMut<'a, V>,
}

impl<'a, T: Enumoid, V> Iterator for EnumSliceIterMut<'a, T, V> {
  type Item = (T, &'a mut V);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let value = self.iter.next()?;
    let key = unsafe { T::from_word_unchecked(self.word) };
    self.word = self.word.inc();
    Some((key, value))
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}

impl<'a, T: Enumoid, V> ExactSizeIterator for EnumSliceIterMut<'a, T, V> {}

impl<'a, T: Enumoid, V> DoubleEndedIterator for EnumSliceIterMut<'a, T, V> {
  #[inline]
  fn next_back(&mut self) -> Option<Self::Item> {
    let value = self.iter.next_back()?;
    let idx = self.word.as_() + self.iter.len();
    let key =
      unsafe { T::from_word_unchecked(T::Word::from_usize_unchecked(idx)) };
    Some((key, value))
  }
}
