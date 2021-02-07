use std::cmp::Ordering;

pub trait OrderedBinarySearch<T>{
  fn lowerbound(&self, x: T) -> Option<usize>;
  fn upperbound(&self, x: T) -> Option<usize>;
}
impl<T: Ord> OrderedBinarySearch<T> for Vec<T> {
  fn lowerbound(&self, x: T) -> Option<usize> {
    let mut low = 0;
    let mut high = self.len();

    while high != low {
      let mid = (low + high)/2;
      match self[mid].cmp(&x) {
        Ordering::Less => {
          low = mid+1;
        }
        Ordering::Greater | Ordering::Equal => {
          high = mid;
        }
      }
    }

    if high == self.len() {
      None
    }else{
      Some(low)
    }
  }

  fn upperbound(&self, x: T) -> Option<usize> {
    let mut low = 0;
    let mut high = self.len();

    while high != low {
      let mid = (low + high)/2;
      match self[mid].cmp(&x) {
        Ordering::Less | Ordering::Equal=> {
          low = mid+1;
        }
        Ordering::Greater => {
          high = mid;
        }
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
  assert_eq!(v.lowerbound(2), Some(1));
  assert_eq!(v.lowerbound(5), Some(4));

  assert_eq!(v.lowerbound(0), Some(0));
  assert_eq!(v.lowerbound(6), None);
}

#[test]
fn lowerbound_test_unorderd_array() {
  let v = vec![3,3,1,5,3];
  assert_eq!(v.lowerbound(3), Some(3));
  assert_eq!(v.lowerbound(1), Some(0));

  assert_eq!(v.lowerbound(6), None);
}

#[test]
fn upperbound_test() {
  let v = vec![1,2,3,4,5];
  assert_eq!(v.upperbound(2), Some(2));

  assert_eq!(v.upperbound(0), Some(0));
  assert_eq!(v.upperbound(5), None);
}

#[test]
fn upperbound_test_unorderd_array() {
  let v = vec![3,1,3,2,5,3];
  assert_eq!(v.upperbound(2), Some(4));
  assert_eq!(v.upperbound(1), Some(2));

  assert_eq!(v.upperbound(4), None);
}
