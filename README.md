# Crude Graph

This is a **very** crude implementation of a Graph Builder and a traverse method to find the path between two nodes.

I made this mainly to learn and didn't use any type of tutorial (which might be quite obvious). Traversal is not optimised and can produce different results each time.

## Usage

### Manual Graph building

Insert all nodes and edges manually.

```rust
let graph = GraphBuilder::<i32>::new()
  .insert_node(1)
  .insert_node(2)
  .insert_edge(1, 2)
  .insert_node(3)
  .insert_edge(1, 3)
  .insert_node(4)
  .insert_edge(4, 3)
  .build();

let path = graph.find_path(1, 4);

/*
Some(
    [
        1,
        3,
        4,
    ],
)
*/
```

### From Nodes

Create a base graph from a list of nodes. Edges will need to be added manually

```rust
let graph = GraphBuilder::<String>::from_nodes(vec![
      "A".into(),
      "B".into(),
      "C".into(),
      "D".into(),
      "E".into(),
      "F".into(),
  ])
  .insert_edge("A".into(), "B".into())
  .insert_edge("A".into(), "D".into())
  .insert_edge("A".into(), "E".into())
  .insert_edge("B".into(), "C".into())
  .insert_edge("D".into(), "E".into())
  .insert_edge("E".into(), "F".into())
  .insert_edge("E".into(), "C".into())
  .insert_edge("C".into(), "F".into())
  .build();

let path = graph.find_path("A".into(), "F".into());

/*
Some(
    [
        "A",
        "D",
        "E",
        "F",
    ],
)
*/
```

### From Edges

```rust
let graph = GraphBuilder::<i32>::from_edges(vec![(1, 2), (1, 3), (4, 3)]).build();

let path = graph.find_path(1, 3);

/*
Some(
    [
        1,
        3,
    ],
)
*/
```