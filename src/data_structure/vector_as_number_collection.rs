use crate::useful_functions::OrderedBinarySearch;

use std::collections::HashMap;
pub struct VectorAsNumberCollection<T> 
{
    map: HashMap<T, usize>,
    arr: Vec<Vec<usize>>
}

impl<T> VectorAsNumberCollection<T> 
    where 
        T: Eq + std::hash::Hash + Copy
{
    pub fn new(vec: &Vec<T>) -> VectorAsNumberCollection<T> {
        let mut size = 0;
        let mut map = HashMap::<T, usize>::new();
        for e in vec.iter() {
            if map.get(e) == None{
                map.insert(*e, size);
                size+=1;
            }
        }
        let mut arr = vec![vec![]; size];
        for (i,e) in vec.iter().enumerate() {
            if let Some(&index) = map.get(e) {
                arr[index].push(i);
            }
        }
        
        VectorAsNumberCollection{
            map,
            arr
        }
    }
    pub fn get_lower_bound(&self, num: T, index: usize) -> Option<usize> {
      if let Some(&map_index) = self.map.get(&num) {
        if let Some(i) = self.arr[map_index].lowerbound(index) {
          Some(self.arr[map_index][i])
        }else{
          None
        }
      }else{
        None
      }
      
    }
    
    pub fn show_vector(&self) -> Vec<Vec<usize>> {
        self.arr.clone()
    }
}

#[test]
fn initialize() {
    let v = VectorAsNumberCollection::new(&vec![1, 2, 1, 3, 4, 4, 2, 3]);
    assert_eq!(v.show_vector(), vec![vec![0,2], vec![1, 6], vec![3, 7], vec![4, 5]]);
}

#[test]
fn find() {
  let v = VectorAsNumberCollection::new(&vec![1, 2, 1, 3, 4, 4, 2, 3]);
  assert_eq!(v.get_lower_bound(3, 3), Some(3));
  assert_eq!(v.get_lower_bound(3, 4), Some(7));
  assert_eq!(v.get_lower_bound(1, 0), Some(0));
  assert_eq!(v.get_lower_bound(1, 3), None);
}
