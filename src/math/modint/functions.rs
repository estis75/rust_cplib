use super::Modulo;

use num::integer::Integer;
impl<T> Modulo<T> 
  where
    T: Integer + Into<i128>  + Into<u128> + Copy
{
  pub fn set_modulo(modulo: T) -> Modulo<T> {
    if modulo == T::zero() {
      panic!("cannot use 0 as modulo")
    }

    let mod_u128: u128 = modulo.into();
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