pub struct UnionFindTree {
  tree: Vec<i32>
}

impl UnionFindTree {
  pub fn init(size: usize) -> UnionFindTree {
    let tree = vec![-1; size];
    UnionFindTree{
      tree
    }
  }

  pub fn is_united(&mut self, l: usize, r: usize) -> bool {
    self.get_root(l as i32) == self.get_root(r as i32)
  }

  fn get_root(&mut self, node: i32) -> usize {
    let node = node as usize;
    if self.tree[node] < 0 {
      node
    }else{
      let root = self.get_root(self.tree[node]);
      self.tree[node] = root as i32;
      root
    }
  }

  pub fn unite(&mut self, l: usize, r: usize) {
    let l = self.get_root(l as i32);
    let r = self.get_root(r as i32);
    if self.is_united(l, r) {return ;}
    if self.tree[l].abs() < self.tree[r].abs() {
      self.tree[r] += self.tree[l];
      self.tree[l] = r as i32;
    }else{
      self.tree[l] += self.tree[r];
      self.tree[r] = l as i32;
    }
  }

  pub fn get_tree(&self) -> Vec<i32> {
    self.tree.clone()
  }
}

#[test]
fn test() {
  let mut u = UnionFindTree::init(8);
  assert_eq!(u.get_tree(), vec![-1; 8]);
  
  u.unite(3, 2);
  u.unite(4, 1);
  assert_eq!(u.get_tree(), vec![-1, 4, 3, -2, -2, -1, -1, -1]);

  u.unite(5, 2);
  assert_eq!(u.get_tree(), vec![-1, 4, 3, -3, -2, 3, -1, -1]);

  u.unite(6,7);
  assert_eq!(u.get_tree(), vec![-1, 4, 3, -3, -2, 3, -2, 6]);

  u.unite(2,7);
  assert_eq!(u.get_tree(), vec![-1, 4, 3, -5, -2, 3, 3, 6]);

  u.unite(2,7);
  assert_eq!(u.get_tree(), vec![-1, 4, 3, -5, -2, 3, 3, 3]);
}