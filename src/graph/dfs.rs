use super::Graph;

impl Graph {
  pub fn dfs(&self, s: usize) -> Vec<Option<i64>> {
    let mut v = vec![None; self.node_size];
    v[s] = Some(0);
    self.dfs_sub(s, v)
  }

  fn dfs_sub(&self, s: usize, mut v: Vec<Option<i64>>) -> Vec<Option<i64>>{
    if let Some(from) = v[s] {
      for e in self.edge[s].iter() {
        if v[e.0] == None {
          v[e.0] = Some(from + e.1);

          v = self.dfs_sub(e.0, v);
        }
      }
    }
    v
  }
}

#[test]
fn test() {
  let mut g = Graph::new(5);
  g.add_edge(0, 1, 3);
  g.add_edge(1, 0, 2);
  g.add_edge(2, 4, 4);
  g.add_edge(2, 1, 9);
  assert_eq!(g.dfs(2), vec![Some(11), Some(9), Some(0), None, Some(4)]);
}