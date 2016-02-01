use std::collections::HashMap;

pub struct Vertex<T> {
    adj: Vec<Option<Box<Vertex<T>>>>, // adjacency list
    name: String,
}

pub struct Graph<T> {
    vertex_map: HashMap<String, <Option<Box<Vertex<T>>>>>,
}

impl<T> Vertex<T> {
    /// Creates a new node with the given name
    fn new(name: String) -> Self {
        let mut node = Vertex {
            adj: Vec::new(),
            name: name,
        };
        node
    }

    /// Adds a vertex to its adjacency list 
    fn add_neighbor(neighbor : Self) {
        adj.push(neighbor);
    }
}

impl<T> Graph<T> {
    fn new() -> Self {
        let mut graph = Graph { 
            vertex_map: HashMap::new(),
        }
    }

    fn add_vertex(name: String) {
        if None(self.vertex_map[name]) {
            self.vertex_map[name] = Vertex::new(name);
        }
    }

    fn add_edge(src: String, dst: String) {
        self.vertex_map[src].add_neighbor();

    }
}
