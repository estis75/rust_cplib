use super::Graph;

impl Graph {
  pub fn bfs(&self, s: usize) -> Vec<Option<i64>> {
    let mut v = vec![None; self.node_size];
    v[s] = Some(0);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(s);

    while !queue.is_empty() {
      if let Some(&tp) = queue.front() {
        if let Some(from) = v[tp] {
          queue.pop_front();
          for e in self.edge[tp].iter() {
            if v[e.0] != None {
              v[e.0] = Some(from + e.1);
              queue.push_back(e.0);
            }
          }
        }
      }
    }
    v
  }

}

#[test]
fn test() {
  let mut g = Graph::new(5);
  g.add_edge(0, 1, 9);
  g.add_edge(0, 2, 2);
  g.add_edge(2, 4, 4);
  g.add_edge(2, 1, 3);
  assert_eq!(g.dfs(0), vec![Some(0), Some(9), Some(2), None, Some(6)]);
}