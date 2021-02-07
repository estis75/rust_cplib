pub struct SegmentTree<T, U> {
  size: usize,
  tree: Vec<U>,
  cmpfn: T,
}

impl<T, U> SegmentTree<T, U> 
  where T: Fn(U, U) -> U ,
        U: Copy + std::fmt::Debug 
  { 
  /// 値の初期化
  pub fn new(n: usize, e: U, arr: Vec<U>, cmpfn: T) -> SegmentTree<T, U> {
    let mut size: usize = 1;
    while size < n  {
      size <<= 1;
    }
    let mut tree: Vec<U> = Vec::<U>::with_capacity(2*size);
    for _ in 0..2*size {
      tree.push(e);
    }
    for (i, val) in arr.iter().enumerate() {
      tree[size + i] = *val;
    }

    for i in (1..size).rev() {
      tree[i] = cmpfn(tree[2*i], tree[2*i+1]);
    }

    SegmentTree{
      size,
      tree,
      cmpfn,
    }
  }

  /// indexの値をnumで更新する.
  pub fn update_number(&mut self, mut index: usize, num: U) {
    self.tree[index + self.size] = num;
    index = (index + self.size) / 2;
    while index > 0 {
      self.tree[index] = (self.cmpfn)(self.tree[2*index], self.tree[2*index+1]);
      index >>= 1;
    }
  }

  /// [l, r)での範囲の答えを得る.
  pub fn get(&self, l: usize, r:usize) -> U {
    // println!("{}, {}, {}", self.size, l, r);
    self.rec(0,self.size,l,r)
  }

  fn rec(&self, min: usize, max: usize, l: usize, r:usize) -> U {
    let mid = (min + max)/2;
    if l == min && r == max {
      return self.tree[self.size/(r-l) + l/(r-l)];
    }

    if l < mid && mid < r{
      (self.cmpfn)(self.rec(min, mid, l, mid) , self.rec(mid, max, mid, r))
    }else if mid <= l {
      self.rec(mid, max, l, r)
    }else if r <= mid {
      self.rec(min, mid, l, r)
    }else{
      panic!("invalid input: min: {}, max: {}, l: {}, r: {}", min, max, l, r );
    }
  }

  pub fn get_by_index(&self, p: usize) -> U {
    if self.size*2 <= p {
      panic!("invalid input: min: {}, max: {}, p: {}", 0, self.size, p );
    }

    self.tree[p]
  }

  pub fn set_by_index(&mut self, p: usize, u: U){
    if self.size*2 <= p {
      panic!("invalid input: min: {}, max: {}, p: {}", 0, self.size, p );
    }

    self.tree[p] = u;
  }

  pub fn update_by_index(&mut self, p: usize){
    if p < self.size {
      let tp = (self.cmpfn)(self.get_by_index(2*p), self.get_by_index(2*p+1));
      self.set_by_index(p, tp);
      // self.set_by_index(p, (self.cmpfn)(tp, self.get_by_index(p)));
    } else if self.size <= p && p < self.size*2 {
      return ;
    } else{
      panic!("invalid input: min: {}, max: {}, p: {}", 0, self.size, p );
    }
  }

  pub fn debug(&self){
    println!("{:?}", self.tree);
  }
}

#[test]
fn it_works() {
    use crate::data_structure::SegmentTree;
    
    let mut ts = SegmentTree::new(3, 0, [1,2,3].to_vec(), |l,r|{l+r});
    ts.update_number(1, 3);
    assert_eq!(ts.tree, vec![0,7,4,3,1,3,3,0]);
}