use super::Graph;
use std::collections::BinaryHeap;

impl Graph {
  pub fn dijkstra(&self, s: usize) -> Vec<Option<i64>> {
    let mut v = vec![None; self.node_size];

    let mut pq = BinaryHeap::new();
    pq.push(Node{node:s, cost:0});
    while let Some(tp) = pq.pop() {
      if v[tp.node] != None { continue; }
      v[tp.node] = Some(tp.cost);
      let from = tp.cost;

      for e in self.edge[tp.node].iter() {
        if let Some(to) = v[e.0] {
          if from + e.1 < to {
            pq.push(Node{node:e.0, cost:from + e.1})
          }
        }else{
          pq.push(Node{node:e.0, cost:from + e.1})
        }
      }
    }
    v
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
  node: usize,
  cost: i64
}

use std::cmp::Ordering;
impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost.cmp(&self.cost)
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[test]
fn test() {
  let mut g = Graph::new(6);
  g.add_edge(0,1,3);
  g.add_edge(0,2,1);
  g.add_edge(2,1,1);
  g.add_edge(2,4,3);
  g.add_edge(3,0,3);

  assert_eq!(g.dijkstra(0), vec![Some(0), Some(2), Some(1), None, Some(4), None]);
}