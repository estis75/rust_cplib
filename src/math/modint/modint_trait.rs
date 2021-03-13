use super::{ModInt, Modulo};
use num::integer::Integer;



use std::convert::From;
impl From<ModInt<'_, i8>> for i8 {
  fn from(orig: ModInt<'_, i8>) -> Self {
    orig.number
  }
}
impl From<ModInt<'_, i16>> for i16 {
  fn from(orig: ModInt<'_, i16>) -> Self {
    orig.number
  }
}
impl From<ModInt<'_, i32>> for i32 {
  fn from(orig: ModInt<'_, i32>) -> Self {
    orig.number
  }
}
impl From<ModInt<'_, i64>> for i64 {
  fn from(orig: ModInt<'_, i64>) -> Self {
    orig.number
  }
}
impl From<ModInt<'_, i128>> for i128 {
  fn from(orig: ModInt<'_, i128>) -> Self {
    orig.number
  }
}
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



use std::ops::{Add, Sub, Mul, Div, Rem};
impl<T> Modulo<T> 
  where 
    T: Integer + Copy
{

  pub fn new(&self, number: T) -> ModInt<T> {
    ModInt{
      number: ((number % self.modulo) + self.modulo) % self.modulo,
      modulo: &self.modulo,
      inv: &self.inv
    }
  }
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

impl Div for ModInt<'_, u32> {
  type Output = Self;

  fn div(self, other: Self) -> Self {
    let index = other.number as usize;
    if let Some(c) = self.inv[index] {
      Self {
        number: (self.number * c) % *self.modulo,
        modulo: self.modulo,
        inv: self.inv
      }
    }else{
      panic!("no inverse element of {:?} on modulo {:?}", other.number, self.modulo)
    }
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
