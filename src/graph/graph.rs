use super::Graph;

impl Graph {
  pub fn new(node_size: usize) -> Graph{
    let mut edge: Vec<Vec<(usize,i64)>> = Vec::<Vec<(usize,i64)>>::with_capacity(node_size);
    for _ in 0..node_size {
      edge.push(vec![]);
    }

    Graph{
      node_size,
      edge_size: 0,
      edge
    }
  }

  pub fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
    self.edge[from].push((to, cost));
    self.edge_size += 1;
  }
}