pub trait Min {
  fn min() -> Self;
}

impl Min for u8 {
  fn min() -> Self {
    std::u8::MIN
  }
}
impl Min for u16 {
  fn min() -> Self {
    std::u16::MIN
  }
}
impl Min for u32 {
  fn min() -> Self {
    std::u32::MIN
  }
}
impl Min for u64 {
  fn min() -> Self {
    std::u64::MIN
  }
}
impl Min for u128 {
  fn min() -> Self {
    std::u128::MIN
  }
}
impl Min for usize {
  fn min() -> Self {
    std::usize::MIN
  }
}

pub struct Modulo<T> {
  number: T,
  modulo: T, 
  inv: Vec<Option<T>>
}

impl Modulo<u32> {
  pub fn set_modulo(modulo: u32) -> Modulo<u32> {
    if modulo == 0 {
      panic!("cannot use 0 as modulo")
    }

    let inv = if modulo < 1e7 as u32 {
      let mut vec = vec![None; modulo as usize];
      vec[1] = Some(1);
      for i in 1..modulo {
        if let Some(c) = vec[(modulo % i) as usize] {
          vec[i as usize] = Some(modulo - c * (modulo / i) % modulo);
        }
      }
      vec
    }else{
      vec![]
    };
    Modulo{number: 0, modulo, inv}
  }
}
impl Modulo<u64> {
  pub fn set_modulo(modulo: u64) -> Modulo<u64> {
    if modulo == 0 {
      panic!("cannot use 0 as modulo")
    }

    let inv = if modulo < 1e7 as u64 {
      let mut vec = vec![None; modulo as usize];
      vec[1] = Some(1);
      for i in 1..modulo {
        if let Some(c) = vec[(modulo % i) as usize] {
          vec[i as usize] = Some(modulo - c * (modulo / i) % modulo);
        }
      }
      vec
    }else{
      vec![]
    };
    Modulo{number: 0, modulo, inv}
  }
}
impl Modulo<usize> {
  pub fn set_modulo(modulo: usize) -> Modulo<usize> {
    if modulo == 0 {
      panic!("cannot use 0 as modulo")
    }

    let inv = if modulo < 1e7 as usize {
      let mut vec = vec![None; modulo];
      vec[1] = Some(1);
      for i in 1..modulo {
        if let Some(c) = vec[modulo % i] {
          vec[i] = Some(modulo - c * (modulo / i) % modulo);
        }
      }
      vec
    }else{
      vec![]
    };
    Modulo{number: 0, modulo, inv}
  }
}

impl<T> Modulo<T> {
  pub fn show_inv(self) -> Vec<Option<T>> {
    self.inv
  }
}

use std::ops::{Add, Sub, Mul, Div, Rem};
impl<T> Modulo<T> 
  where 
    T: Min 
        + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Rem<Output=T> + Div<Output=T> 
        + PartialEq + Copy + PartialEq
{

  pub fn new(&self, number: T) -> ModInt<T> {
    ModInt{
      number: ((number % self.modulo) + self.modulo) % self.modulo,
      modulo: &self.modulo,
      inv: &self.inv
    }
  }
}


pub struct ModInt<'a, T> {
  number: T, 
  modulo: &'a T,
  inv: &'a Vec<Option<T>>
}

impl<T> Add for ModInt<'_, T> 
  where 
    T: Add<Output=T> + Rem<Output=T> + Copy
{
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      number: (self.number + other.number + *self.modulo) % *self.modulo,
      modulo: self.modulo,
      inv: self.inv
    }
  }
}
impl<T> Sub for ModInt<'_, T>
  where 
    T: Add<Output=T> + Sub<Output=T> + Rem<Output=T> + Copy
{
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      number: (self.number + *self.modulo - other.number) % *self.modulo,
      modulo: self.modulo,
      inv: self.inv
    }
  }
}
impl<T> Mul for ModInt<'_, T>
  where 
    T: Mul<Output=T> + Rem<Output=T> + Copy
{
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    Self {
      number: (self.number * other.number) % *self.modulo,
      modulo: self.modulo,
      inv: self.inv
    }
  }
}

impl<T> Div for ModInt<'_, T> 
  where 
    T: Mul<Output=T> + Rem<Output=T> + Copy + std::fmt::Debug + std::convert::TryInto<usize>
{
  type Output = Self;

  fn div(self, other: Self) -> Self {
    if let Ok(index) = other.number.try_into() {
      if let Some(c) = self.inv[index] {
        Self {
          number: (self.number * c) % *self.modulo,
          modulo: self.modulo,
          inv: self.inv
        }
      }else{
        panic!("no inverse element of {:?} on modulo {:?}", other.number, self.modulo)
      }
    }else{
      panic!("cannot convert from diviser")
    }
  }
}

use std::convert::From;
impl From<ModInt<'_, u8>> for u8 {
  fn from(orig: ModInt<'_, u8>) -> Self {
    orig.number
  }
}
impl From<ModInt<'_, u16>> for u16 {
  fn from(orig: ModInt<'_, u16>) -> Self {
    orig.number
  }
}
impl From<ModInt<'_, u32>> for u32 {
  fn from(orig: ModInt<'_, u32>) -> Self {
    orig.number
  }
}
impl From<ModInt<'_, u64>> for u64 {
  fn from(orig: ModInt<'_, u64>) -> Self {
    orig.number
  }
}
impl From<ModInt<'_, u128>> for u128 {
  fn from(orig: ModInt<'_, u128>) -> Self {
    orig.number
  }
}
impl From<ModInt<'_, usize>> for usize {
  fn from(orig: ModInt<'_, usize>) -> Self {
    orig.number
  }
}


impl<T> std::cmp::PartialEq for ModInt<'_, T>
  where
    T: PartialEq
{
  fn eq(&self, other: &Self) -> bool {
    self.number == other.number && self.modulo == other.modulo
  }
}

impl<T> std::fmt::Debug for ModInt<'_, T> 
  where 
    T: std::fmt::Debug
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ModInt")
      .field("number", &self.number)
      .field("modulo", self.modulo)
      .field("modulo", self.inv)
      .finish()
  }

}

#[test]
fn test_setup_and_convert() {
  let m = Modulo::<u64>::set_modulo(10);
  let v = m.new(3);
  let n: u64 = v.into();
  assert_eq!(n, 3);
}

#[test]
fn test_set_modulo() {
  let m = Modulo::<u64>::set_modulo(10);
  assert_eq!(m.show_inv(), vec![None, Some(1), None, Some(7), None, None, None, Some(3), None, Some(9)]);
}

#[test]
fn test_arithmetic_operations() {
  let m = Modulo::<u32>::set_modulo(11);
  let c1 = m.new(3);
  let c2 = m.new(5);
  let c3 = m.new(8);
  let c4 = m.new(9);
  let c: u32 = (c1 + c2).into();
  assert_eq!(c, 8);
  let c: u32 = (c3 - c4).into();
  assert_eq!(c, 10);
  let c1 = m.new(3);
  let c2 = m.new(5);
  let c3 = m.new(8);
  let c4 = m.new(9);
  let c: u32 = (c1 * c2).into();
  assert_eq!(c, 4);
  let c: u32 = (c3 / c4).into();
  assert_eq!(c, 7);
}