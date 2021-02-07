mod bfs;
mod dfs;
mod graph;

pub struct Graph {
  node_size: usize,
  edge_size: usize,
  edge: Vec<Vec<(usize, i64)>>,
}
