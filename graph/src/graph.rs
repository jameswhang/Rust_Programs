use std::collections::{HashSet, HashMap};
use std::hash::{Hash, SipHasher, Hasher};

#[derive(Eq, Debug)]
pub struct Vertex<'a, T> {
    adj: HashSet<&'a Vertex<'a, T>>,
    key : T<'a>',
}

impl<'a, T : Eq> Vertex<'a, T> {
    /// Creates a new node with the given name
    pub fn new(key: T) -> Vertex<'a, T> {
        Vertex {
            adj: HashSet::new(),
            key: key,
        }
    }

    fn add_neighbor<'b, 'c>(a : &Vertex<'b, T>, b : &Vertex<'c, T>) {
        Vertex::add_link(a, b);
        Vertex::add_link(b, a);
    }

    fn add_link<'b, 'c>(a : &Vertex<'b, T>, b : &Vertex<'c, T>) {
        a.adj.insert(b);
    }
}


impl<'a, T : Eq> PartialEq for Vertex<'a, T> {
    fn eq(&self, other : &Vertex<T>) -> bool {
        (self.key == other.key)
    }

    fn ne(&self, other : &Vertex<T>) -> bool{
        (self.key != other.key)
    }
}

impl<'a, T : Eq> Hash for Vertex<'a, T> {
    fn hash<H : Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}




pub struct Graph<'a, 'b, T> {
    vertex_map: Option<HashMap<String, Vertex<'b, T>>>,
}


impl<'a, 'b, T> Graph<'a, 'b, T> {
    fn new() -> Graph<'a, 'b, T> {
        Graph {
            vertex_map: HashMap::new(),
        }
    }

    fn add_vertex(self, name: String) {
        unimplemented!();

        // match self.vertex_map[name] {
        //     None => {
        //         self.vertex_map[name] = Vertex::new(name);
        //     }
        // }
    }

    fn add_edge(self, src: String, dst: String) {
        unimplemented!();

        // let source = self.vertex_map[src];
        // let destination = self.vertex_map[dst];
        // self.vertex_map[src].add_neighbor(destination);
        // self.vertex_map[src].add_neighbor(source);
    }

    fn search_path(src: String, dst:String) -> Option<Vec<&'b Vertex<'b, T>>> {
        unimplemented!();
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
