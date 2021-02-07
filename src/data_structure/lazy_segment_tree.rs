use super::SegmentTree;
pub struct LazySegmentTree<S, T, U, V, W> {
  size: usize,
  tree: SegmentTree<S,U>,
  operators: Vec<T>,
  id_op: T,
  composite_op_and_ob: V,
  composite_operators: W,
}

/// refs: https://beet-aizu.hatenablog.com/entry/2017/12/01/225955

impl<S, T, U, V, W> LazySegmentTree<S, T, U, V, W>
  where 
    U: Copy + std::fmt::Debug,
    S: Fn(U, U) -> U ,
    T: Copy + Eq + std::fmt::Debug,
    V: Fn(T, U) -> U,
    W: Fn(T, T) -> T,
{
  /// 初期化
  pub fn new(n: usize, e: U, arr: Vec<U>, cmpfn: S,
      id_op: T, composite_op_and_ob: V, composite_operators: W 
    ) -> LazySegmentTree<S, T, U, V, W> {
    let tree = SegmentTree::new(n,e,arr,cmpfn);

    let mut size: usize = 1;
    while size < n  {
      size <<= 1;
    }
    let mut operators: Vec<T> = Vec::<T>::with_capacity(2*size);
    for _ in 0..2*size {
      operators.push(id_op);
    }

    LazySegmentTree{
      size,
      tree,
      operators,
      id_op,
      composite_op_and_ob,
      composite_operators
    }
  }
  
  /// [l, r)の範囲の作用を伝播する.
  fn eval(&mut self, k: usize) {
    if self.operators[k] == self.id_op  {
      return;
    }

    if k < self.size {
      self.operators[2*k] = (self.composite_operators)(self.operators[2*k], self.operators[k]);
      self.operators[2*k+1] = (self.composite_operators)(self.operators[2*k+1], self.operators[k]);
    }
    // println!("{}", k);
    self.tree.set_by_index(k, (self.composite_op_and_ob)(self.operators[k], self.tree.get_by_index(k)));
    // print!("{}: {:?}", k, (self.composite_op_and_ob)(self.operators[k], self.tree.get_by_index(k)));
    self.operators[k] = self.id_op;
  }

  /// [l, r)の計算をする.
  pub fn get(&mut self, l: usize, r:usize) -> U {
    if self.size < r {
      panic!("invalid input: min: {}, max: {}, l: {}, r: {}", 0, self.size, l, r );
    }

    self.rec_g(0, self.size, l, r);
    self.tree.get(l,r)
  }

  fn rec_g(&mut self, min: usize, max: usize, l: usize, r:usize) {
      // println!("{}:{}", l, r);
    let index = self.size/(max-min) + min/(max-min);
    self.eval(index);

    let mid = (min + max)/2;
    if l == min && r == max {
      return ;
    }

    if l < mid && mid < r{
      self.rec_g(min, mid, l, mid); self.rec_g(mid, max, mid, r);
    }else if mid <= l {
      self.rec_g(mid, max, l, r);
    }else if r <= mid {
      self.rec_g(min, mid, l, r);
    }else{
      panic!("invalid input: min: {}, max: {}, l: {}, r: {}", min, max, l, r );
    }

    // self.tree.update_by_index(index);
  }

  pub fn set_operator(&mut self, l: usize, r:usize, op: T) {
    if self.size < r {
      panic!("invalid input: min: {}, max: {}, l: {}, r: {}", 0, self.size, l, r );
    }

    self.rec_s(0, self.size, l, r, op);
  }

  fn rec_s(&mut self, min: usize, max: usize, l: usize, r:usize, op:T) {
    let index = self.size/(max-min) + min/(max-min);

    let mid = (min + max)/2;
    if l == min && r == max {
      self.operators[index] = (self.composite_operators)(op, self.operators[index]);
      self.eval(index);
      return ;
    }

    self.eval(index);
    if l < mid && mid < r{
      self.rec_s(min, mid, l, mid, op); self.rec_s(mid, max, mid, r, op);
    }else if mid <= l {
      self.eval(self.size/(mid-min) + min/(mid-min)); self.rec_s(mid, max, l, r, op);
    }else if r <= mid {
      self.rec_s(min, mid, l, r, op); self.eval(self.size/(max-mid) + mid/(max-mid)); 
    }else{
      panic!("invalid input: min: {}, max: {}, l: {}, r: {}", min, max, l, r );
    }

    self.tree.update_by_index(index);
  }
  
  pub fn debug(&self) {
    self.tree.debug();
    println!("{:?}", self.operators);
  }

}

#[test]
fn it_works() {
    use crate::data_structure::LazySegmentTree;
    
    let mut ts = LazySegmentTree::new(3, 0, [3,9,2].to_vec(), num::integer::gcd, 0, num::integer::gcd, num::integer::gcd);
    ts.set_operator(1, 2, 3);
    assert_eq!(ts.get(0,3), 1);
}