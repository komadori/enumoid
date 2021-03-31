use enumoid::Enumoid;

#[derive(Debug, Enumoid, PartialEq)]
pub enum Zero {}

#[derive(Debug, Enumoid, PartialEq)]
pub enum Three {
  A,
  B,
  C,
}

#[derive(Debug, Enumoid, PartialEq)]
pub enum Sixteen {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
}

#[derive(Debug, Enumoid, PartialEq)]
pub enum Seventeen {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
}
