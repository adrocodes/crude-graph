use std::{
    cmp::{Eq, PartialEq},
    collections::{HashMap, HashSet},
    hash::Hash,
};

fn main() {
    let graph = GraphBuilder::<i32>::new()
        .insert_node(1)
        .insert_node(2)
        .insert_edge(1, 2)
        .insert_node(3)
        .insert_edge(1, 3)
        .insert_node(4)
        .insert_edge(4, 3)
        .build();

    dbg!(&graph);

    let graph = GraphBuilder::<String>::from_vertices(vec![
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

    dbg!(&graph);
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Node<T>(T);

#[derive(Debug)]
struct GraphBuilder<T: PartialEq + Eq + Hash + Clone> {
    vertices: HashMap<T, HashSet<Node<T>>>,
}

#[derive(Debug)]
struct Graph<T: PartialEq + Eq + Hash + Clone> {
    vertices: HashMap<T, HashSet<Node<T>>>,
}

impl<T: PartialEq + Eq + Hash + Clone> Default for GraphBuilder<T> {
    fn default() -> Self {
        GraphBuilder {
            vertices: HashMap::new(),
        }
    }
}

impl<T: PartialEq + Eq + Hash + Clone> GraphBuilder<T> {
    fn new() -> Self {
        GraphBuilder::default()
    }

    fn _insert_node(builder: &mut GraphBuilder<T>, node: T) {
        builder.vertices.insert(node, HashSet::new());
    }

    pub fn insert_node(mut self, node: T) -> GraphBuilder<T> {
        GraphBuilder::<T>::_insert_node(&mut self, node);
        self
    }

    fn _insert_edge(builder: &mut GraphBuilder<T>, from: &T, to: &T) {
        let has_from = builder.vertices.contains_key(&from);
        let has_to = builder.vertices.contains_key(&to);

        if !has_from || !has_to {
            return;
        }

        if let Some(from_vert) = builder.vertices.get_mut(&from) {
            from_vert.insert(Node(to.clone()));
        }

        if let Some(to_vert) = builder.vertices.get_mut(&to) {
            to_vert.insert(Node(from.clone()));
        }
    }

    pub fn insert_edge(mut self, from: T, to: T) -> GraphBuilder<T> {
        GraphBuilder::<T>::_insert_edge(&mut self, &from, &to);

        self
    }

    pub fn from_vertices(nodes: Vec<T>) -> GraphBuilder<T> {
        let mut graph = GraphBuilder::<T>::new();

        for node in nodes {
            GraphBuilder::<T>::_insert_node(&mut graph, node);
        }

        graph
    }

    pub fn build(self) -> Graph<T> {
        Graph {
            vertices: self.vertices,
        }
    }
}

impl<T: PartialEq + Eq + Hash + Clone> Graph<T> {}
