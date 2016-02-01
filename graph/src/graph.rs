use std::collections::HashMap;

pub struct Vertex<T> {
    adj: Vec<Option<Box<Vertex<T>>>>, // adjacency list
    name: Option<T>,
}

pub struct Graph<T> {
    vertex_map: HashMap<String, Vertex<T>>, //<Option<Box<Vertex<T>>>>>,
}

impl<T> Vertex<T> {
    /// Creates a new node with the given name
    fn new(name: Option<T>) -> Self {
        let mut node = Vertex {
            adj: Vec::new(),
            name: name,
        };
        node
    }

    /// Adds a vertex to its adjacency list 
    fn add_neighbor(self, neighbor : Self) {
        self.adj.push(neighbor);
    }
}

impl<T> Graph<T> {
    fn new() -> Self {
        let mut graph = Graph { 
            vertex_map: HashMap::new(),
        };
        graph
    }

    fn add_vertex(self, name: String) {
        match self.vertex_map[name] {
            None => {
                self.vertex_map[name] = Vertex::new(name);
            }
        }
    }

    fn add_edge(self, src: String, dst: String) {
        let source = self.vertex_map[src];
        let destination = self.vertex_map[dst];
        self.vertex_map[src].add_neighbor(destination);
        self.vertex_map[src].add_neighbor(source);
    }

    fn search_path(src: String, dst:String) -> Option<Vec<Vertex<T>>> {
        // TODO;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_graph() {
        assert_eq!(1, 1);
    }
    
}
