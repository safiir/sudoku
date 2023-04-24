// pub(crate) use slice;

use radix::RadixNum;

#[allow(unused_macros)]
#[macro_export]
macro_rules! debug {
  ($exp:expr) => {
    println!("{:?}", $exp)
  };
}

#[macro_export]
macro_rules! slice {
  ([$($expr:expr),*]) => {
    &[
      $(
        & $expr[..]
      ),*
    ][..]
  };
}

pub fn vectorify(matrix: &[&[char]]) -> Vec<Vec<char>> {
  matrix.iter().map(|xs| xs.to_vec()).collect()
}

pub fn char_to_radix(ch: u8, len: usize) -> char {
  RadixNum::from(ch)
    .with_radix(len)
    .unwrap()
    .as_str()
    .chars()
    .last()
    .unwrap()
}

pub struct Space {
  pub pos: (usize, usize),
  pub opts: Vec<u8>,
}
