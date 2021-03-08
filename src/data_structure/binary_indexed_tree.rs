pub struct BinaryIndexedTree<T, U> {
  data: Vec<T>,
  calc: U
}

impl<T, U> BinaryIndexedTree<T, U> 
  where
    T: Copy,
    U: Fn(T, T) -> T
{
  pub fn new(size: usize, e: T, calc: U) -> BinaryIndexedTree<T,U> {
    let data = vec![e; size+1];
    BinaryIndexedTree{ data, calc } 
  }

  pub fn add(&mut self, index: usize, num: T) {
    if index >= self.data.len() {
      panic!("index out of range: {}", self.data.len()/2);
    }
    self.add_rec(index + 1, num);
  }

  fn add_rec(&mut self, index: usize, num: T) {
    if index < self.data.len() {
      self.data[index] = (self.calc)(self.data[index], num);
      self.add_rec(index + (index & (!index + 1)) , num);
    }
  }

  // index以下の総和を返す
  pub fn get(&self, index: usize) -> T {
    if index >= self.data.len() {
      panic!("index out of range: {}", self.data.len()/2);
    }
    self.get_rec(index + 1)
  }

  fn get_rec(&self, index: usize) -> T {
    if index == 0 { 
      self.data[0]
    }else{
      let c = index & (!index + 1);
      (self.calc)(self.data[index], self.get(index - c))
    }
  }

  pub fn debug(&self) -> Vec<T> {
    self.data.clone()
  }

}

#[test]
fn add_test() {
  let mut bit = BinaryIndexedTree::new(6, 0, |x, y| x + y);
  assert_eq!(bit.debug(), vec![0,0,0,0,0,0,0]);
  bit.add(3, 2);
  assert_eq!(bit.debug(), vec![0,0,0,0,2,0,0]);
  bit.add(1, 3);
  assert_eq!(bit.debug(), vec![0,0,3,0,5,0,0]);
  bit.add(0, 1);
  assert_eq!(bit.debug(), vec![0,1,4,0,6,0,0]);
  bit.add(4, 1);
  assert_eq!(bit.debug(), vec![0,1,4,0,6,1,1]);
}
#[test]
fn get_test() {
  let mut bit = BinaryIndexedTree::new(6, 0, |x, y| x + y);
  assert_eq!(bit.debug(), vec![0,0,0,0,0,0,0]);
  bit.add(3, 2);
  assert_eq!(bit.get(3), 0);
  // assert_eq!(bit.get(3), 2);
  // assert_eq!(bit.get(4), 2);
  assert_eq!(bit.debug(), vec![0,0,0,0,2,0,0]);
  bit.add(1, 3);
  assert_eq!(bit.debug(), vec![0,0,3,0,5,0,0]);
  bit.add(0, 1);
  assert_eq!(bit.debug(), vec![0,1,4,0,6,0,0]);
  bit.add(4, 1);
  assert_eq!(bit.debug(), vec![0,1,4,0,6,1,1]);
}