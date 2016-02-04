use std::collections::{HashSet, HashMap};
use std::hash::{Hash, SipHasher, Hasher};

#[derive(Eq, Debug)]
pub struct Vertex<'a> {
    adj: HashSet<&'a Vertex<'a>>,
    key: String,
}

pub struct Graph<'a> {
    vertex_map: HashMap<String, &'a Vertex<'a>>,
}

impl <'a> Vertex<'a> {
    pub fn new(key: String) -> Vertex<'a> {
        Vertex {
            adj: HashSet::new(),
            key: key,
        }
    }

    pub fn add_neighbor(&mut self, a: &'a Vertex) {
        Vertex::add_link(self, a);
    }

    pub fn is_neighbor(self, a: &'a Vertex) -> bool {
        return self.adj.contains(a);
    }

    fn add_link(&mut self, a: &'a Vertex) {
        self.adj.insert(a);
    }
}

impl <'a> PartialEq for Vertex<'a> {
    fn eq(&self, other: &Vertex<'a>) -> bool {
        (self.key == other.key)
    }

    fn ne(&self, other: &Vertex<'a>) -> bool {
        (self.key != other.key)
    }
}

impl <'a> Hash for Vertex<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl <'a> Graph<'a> {
    pub fn new() -> Graph<'a> {
        Graph {
            vertex_map: HashMap::new(),
        }
    }

    pub fn add_vertex(self, name:String) {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use graph::{Vertex, Graph};
}
