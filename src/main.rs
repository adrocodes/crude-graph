use std::{
    cmp::{Eq, PartialEq},
    collections::HashSet,
    hash::Hash,
};

fn main() {
    let graph = GraphBuilder::<i32>::new()
        .insert_node(Node(1))
        .insert_node(Node(2))
        .insert_edge(1, 2)
        .insert_node(Node(3))
        .insert_edge(1, 3)
        .insert_node(Node(4))
        .insert_edge(4, 3)
        .build();

    dbg!(&graph);
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Node<T>(T);

#[derive(Debug, Hash, Eq, PartialEq)]
struct Edge<T> {
    from: Node<T>,
    to: Node<T>,
}

#[derive(Debug)]
struct GraphBuilder<T: PartialEq + Eq + Hash + Copy> {
    nodes: HashSet<Node<T>>,
    edges: HashSet<Edge<T>>,
}

#[derive(Debug)]
struct Graph<T: PartialEq + Eq + Hash + Copy> {
    nodes: HashSet<Node<T>>,
    edges: HashSet<Edge<T>>,
}

impl<T: PartialEq + Eq + Hash + Copy> Default for GraphBuilder<T> {
    fn default() -> Self {
        GraphBuilder {
            nodes: HashSet::new(),
            edges: HashSet::new(),
        }
    }
}

impl<T: PartialEq + Eq + Hash + Copy> GraphBuilder<T> {
    fn new() -> Self {
        GraphBuilder::default()
    }

    fn insert_node(mut self, node: Node<T>) -> GraphBuilder<T> {
        self.nodes.insert(node);
        self
    }

    fn find_node(&self, id: T) -> Option<&Node<T>> {
        let node = self.nodes.iter().find(|n| n.0 == id);
        node
    }

    fn insert_edge(mut self, from: T, to: T) -> GraphBuilder<T> {
        let from_node = self.find_node(from);
        let to_node = self.find_node(to);

        if from_node.is_none() || to_node.is_none() {
            return self;
        }

        let from_node = from_node.unwrap();
        let to_node = to_node.unwrap();

        let edge = Edge {
            from: Node(from_node.0),
            to: Node(to_node.0),
        };

        self.edges.insert(edge);

        self
    }

    fn build(self) -> Graph<T> {
        Graph {
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}

impl<T: PartialEq + Eq + Hash + Copy> Graph<T> {}
