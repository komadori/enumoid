use enumoid::Enumoid;
use enumoid::generate_enumoid;
use std::fmt::Debug;

/// Exposes the canonical, in-order list of every value inhabiting a type. The
/// ordering tests use this as the golden reference against which conversions
/// and navigation are validated.
pub trait GoldenValues: Enumoid + Copy + Debug + PartialEq + 'static {
  const VALUES: &'static [Self];
}

#[derive(Copy, Clone, Debug, Enumoid, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Three {
  A,
  B,
  C,
}

impl GoldenValues for Three {
  const VALUES: &'static [Self] = &[Three::A, Three::B, Three::C];
}

#[derive(Copy, Clone, Debug, Enumoid, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[index_type(u32)]
#[bitset_word_types(u8, u16, usize)]
pub enum WideThree {
  A,
  B,
  C,
}

impl GoldenValues for WideThree {
  const VALUES: &'static [Self] = &[WideThree::A, WideThree::B, WideThree::C];
}

#[derive(Copy, Clone, Debug, Enumoid, PartialEq)]
pub struct StructOne;

impl GoldenValues for StructOne {
  const VALUES: &'static [Self] = &[StructOne];
}

#[derive(Copy, Clone, Debug, Enumoid, PartialEq)]
pub struct StructThree(pub Three);

impl GoldenValues for StructThree {
  const VALUES: &'static [Self] = &[
    StructThree(Three::A),
    StructThree(Three::B),
    StructThree(Three::C),
  ];
}

#[derive(Copy, Clone, Debug, Enumoid, PartialEq)]
pub enum CompoundSeven {
  X(Three),
  Y,
  Z(Three),
}

impl GoldenValues for CompoundSeven {
  const VALUES: &'static [Self] = &[
    CompoundSeven::X(Three::A),
    CompoundSeven::X(Three::B),
    CompoundSeven::X(Three::C),
    CompoundSeven::Y,
    CompoundSeven::Z(Three::A),
    CompoundSeven::Z(Three::B),
    CompoundSeven::Z(Three::C),
  ];
}

#[derive(Copy, Clone, Debug, Enumoid, PartialEq)]
pub enum CompoundOnWideSeven {
  X(WideThree),
  Y,
  Z(WideThree),
}

impl GoldenValues for CompoundOnWideSeven {
  const VALUES: &'static [Self] = &[
    CompoundOnWideSeven::X(WideThree::A),
    CompoundOnWideSeven::X(WideThree::B),
    CompoundOnWideSeven::X(WideThree::C),
    CompoundOnWideSeven::Y,
    CompoundOnWideSeven::Z(WideThree::A),
    CompoundOnWideSeven::Z(WideThree::B),
    CompoundOnWideSeven::Z(WideThree::C),
  ];
}

#[derive(Copy, Clone, Debug, Enumoid, PartialEq)]
#[index_type(u32)]
pub enum CompoundWideOnSeven {
  X(Three),
  Y,
  Z(Three),
}

impl GoldenValues for CompoundWideOnSeven {
  const VALUES: &'static [Self] = &[
    CompoundWideOnSeven::X(Three::A),
    CompoundWideOnSeven::X(Three::B),
    CompoundWideOnSeven::X(Three::C),
    CompoundWideOnSeven::Y,
    CompoundWideOnSeven::Z(Three::A),
    CompoundWideOnSeven::Z(Three::B),
    CompoundWideOnSeven::Z(Three::C),
  ];
}

#[derive(Copy, Clone, Debug, Enumoid, PartialEq)]
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

impl GoldenValues for Sixteen {
  const VALUES: &'static [Self] = &[
    Sixteen::A,
    Sixteen::B,
    Sixteen::C,
    Sixteen::D,
    Sixteen::E,
    Sixteen::F,
    Sixteen::G,
    Sixteen::H,
    Sixteen::I,
    Sixteen::J,
    Sixteen::K,
    Sixteen::L,
    Sixteen::M,
    Sixteen::N,
    Sixteen::O,
    Sixteen::P,
  ];
}

#[derive(Copy, Clone, Debug, Enumoid, PartialEq)]
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

impl GoldenValues for Seventeen {
  const VALUES: &'static [Self] = &[
    Seventeen::A,
    Seventeen::B,
    Seventeen::C,
    Seventeen::D,
    Seventeen::E,
    Seventeen::F,
    Seventeen::G,
    Seventeen::H,
    Seventeen::I,
    Seventeen::J,
    Seventeen::K,
    Seventeen::L,
    Seventeen::M,
    Seventeen::N,
    Seventeen::O,
    Seventeen::P,
    Seventeen::Q,
  ];
}

generate_enumoid!(
  #[derive(Copy, Clone, Debug, PartialEq)]
  #[index_type(u16)]
  pub ThreeHundred,
  A,
  1..=300
);
