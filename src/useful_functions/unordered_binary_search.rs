pub trait UnorderedBinarySearch<T, U>
{
  // cmp は等号を含まないものとして使う
  fn lowerbound(&self, x: T, cmp: U) -> Option<usize>;
  fn upperbound(&self, x: T, cmp: U) -> Option<usize>;
}
impl<T, U> UnorderedBinarySearch<T, U> for Vec<T> 
  where
    U: Fn(&T, &T) -> bool
{
  fn lowerbound(&self, x: T, cmp: U) -> Option<usize> {
    let mut low = 0;
    let mut high = self.len();

    while high != low {
      let mid = (low + high)/2;
      if cmp(&self[mid], &x) {
        low = mid+1;
      }else{
        high = mid;
      }
    }

    if high == self.len() {
      None
    }else{
      Some(low)
    }
  }

  fn upperbound(&self, x: T, cmp: U) -> Option<usize> {
    let mut low = 0;
    let mut high = self.len();

    while high != low {
      let mid = (low + high)/2;
      if !cmp(&x, &self[mid]) {
        low = mid+1;
      }else{
        high = mid;
      }
    }

    if high == self.len() {
      None
    }else{
      Some(low)
    }
  }
}

#[test]
fn lowerbound_test() {
  let v = vec![1,2,3,4,5];
  assert_eq!(v.lowerbound(2, |x,y|{x<y}), Some(1));
  assert_eq!(v.lowerbound(5, |x,y|{x<y}), Some(4));

  assert_eq!(v.lowerbound(0, |x,y|{x<y}), Some(0));
  assert_eq!(v.lowerbound(6, |x,y|{x<y}), None);
}

#[test]
fn lowerbound_test_unorderd_array() {
  let v = vec![3,3,1,5,3];
  assert_eq!(v.lowerbound(3, |x,y|{x<y}), Some(3));
  assert_eq!(v.lowerbound(1, |x,y|{x<y}), Some(0));

  assert_eq!(v.lowerbound(6, |x,y|{x<y}), None);
}

#[test]
fn upperbound_test() {
  let v = vec![1,2,3,4,5];
  assert_eq!(v.upperbound(2, |x,y|{x<y}), Some(2));

  assert_eq!(v.upperbound(0, |x,y|{x<y}), Some(0));
  assert_eq!(v.upperbound(5, |x,y|{x<y}), None);
}

#[test]
fn upperbound_test_unorderd_array() {
  let v = vec![3,1,3,2,5,3];
  assert_eq!(v.upperbound(2, |x,y|{x<y}), Some(4));
  assert_eq!(v.upperbound(1, |x,y|{x<y}), Some(2));

  assert_eq!(v.upperbound(4, |x,y|{x<y}), None);
}
