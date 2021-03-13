pub struct Modulo<T> {
  modulo: T, 
  inv: Vec<Option<u32>>
}

pub struct ModInt<'a, T> {
  number: T, 
  modulo: &'a T,
  inv: &'a Vec<Option<u32>>
}

mod modint_trait;
mod functions;