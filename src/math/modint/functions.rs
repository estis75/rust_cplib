use super::{Modulo, ModInt};

use num::integer::Integer;
impl<T> Modulo<T> 
  where
    T: Integer + Into<i128> + Copy
{
  pub fn set_modulo(modulo: T) -> Modulo<T> {
    if modulo <= T::zero() {
      panic!("cannot use 0 or less number as modulo")
    }

    let mod_u128: i128 = modulo.into();
    let inv = if mod_u128 <= 10_000_000 {
      let mod_usize: usize = mod_u128 as usize;
      let mut vec = vec![None; mod_usize];
      vec[1] = Some(1u32);
      for i in 1..mod_usize {
        if let Some(c) = vec[mod_usize % i] {
          let modulo = mod_usize as u32;
          let num = modulo - c * (modulo / i as u32) % modulo;
          vec[i] = Some(num);
        }
      }
      vec
    }else{
      vec![]
    };
    Modulo{modulo, inv}
  }
}

impl Modulo<u32> {
  pub fn show_inv(self) -> Vec<Option<u32>> {
    self.inv
  }
}

impl<T> ModInt<'_, T> 
  where 
    T: Integer + Into<i128> + std::fmt::Display + Copy
{
  pub fn chinese_reduction_theorem(self, other: Self) -> T {
    use num::integer::gcd;
    let modulo_gcd = gcd(*self.modulo, *other.modulo);
    let number_diff = self.number - other.number;
    if number_diff % modulo_gcd != T::zero() {
      panic!("no such number: x = {} mod {} and = {} mod {}", self.number, self.modulo, other.number, other.modulo)
    }else{
      let ret = ModInt::extended_euclidean_algorithm(*self.modulo, *other.modulo);
      let product_modulo = *self.modulo * *other.modulo;

      let new_modulo = product_modulo / modulo_gcd;

      ((self.number + *self.modulo * number_diff / modulo_gcd * ret.0) % new_modulo + new_modulo) % new_modulo
    }
  }
}

impl<T> ModInt<'_, T> 
  where 
    T: Integer + Into<i128> + std::fmt::Display + Copy
{
  /// 負の値で帰ってくるのって嫌な気持ちになるので賢くしてほしいね
  pub fn extended_euclidean_algorithm(l: T, r: T) -> (T, T) {
    if r > l {
      ModInt::extended_euclidean_algorithm(r, l)
    }else{
      if r == T::zero() {
        (T::one(), T::zero())
      }else{
        let ret = ModInt::extended_euclidean_algorithm(r, l%r);
        (ret.1, ret.0 - l/r*ret.1)
      }
    }
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
  let m = Modulo::set_modulo(10);
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

#[test]
fn test_exteuc() {
  assert_eq!(ModInt::extended_euclidean_algorithm(1,0), (1, 0));
  assert_eq!(ModInt::extended_euclidean_algorithm(2,1), (0, 1));
  assert_eq!(ModInt::extended_euclidean_algorithm(3,2), (1, -1));

  assert_eq!(ModInt::extended_euclidean_algorithm(57, 13), (-5, 22))
}

#[test]
fn test_crt() {
  let l = Modulo::set_modulo(15);
  let r = Modulo::set_modulo(10);
  
  let ln = l.new(4);
  let rn = r.new(9);
  assert_eq!(ln.chinese_reduction_theorem(rn), 19);
}