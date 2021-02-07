# グラフ

## [struct Graph](./graph.rs)

グラフの構造体を定義している.
これからのグラフ次第ではこれを加工していくかも.

## [bfs](./bfs.rs)

深さ優先探索.  再帰.
始点を渡してVec<Option<i64>>が帰ってくる.

## [dfs](./dfs.rs)

幅優先探索.
始点を渡してVec<Option<i64>>が帰ってくる.

## [dijkstra](./dijkstra.rs)

ダイクストラ法を用いた最短距離探索.
始点を渡してVec<Option<i64>>が帰ってくる.

reference: https://doc.rust-lang.org/std/collections/binary_heap/index.html