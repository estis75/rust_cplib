# データ構造

## [segment_tree](./segment_tree.rs)

セグ木.  2^nのサイズを確保する方.

## [lazy_segment_tree](./lazy_segment_tree.rs)

遅延セグ木.  セグ木の構造を使いまわしてるので, 壊れそうで怖い.

## [union_find_tree](./union_find_tree.rs)

unionfind木.  rootのノードにはその木のサイズを負にしたものを乗っけてるけど, これじゃまずいなにかがある？

## [binary_indexed_tree](./binary_indexed_tree.rs)

BIT.  1-indexed.