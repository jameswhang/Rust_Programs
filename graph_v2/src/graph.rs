use std::collections::{HashSet, HashMap, VecDeque};
use std::cell::{RefCell};
use std::rc::Rc;
use std::hash::{Hash, Hasher};

pub struct Graph{
    vertices: HashMap<String, VertexCell>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, name : String) {
        self.vertices.entry(name.clone()).or_insert(Vertex::new_cell(name));
    }

    pub fn add_vertices(&mut self, names: Vec<String>) {
        for name in names {
            self.add_vertex(name);
        }
    }

    pub fn add_edge(&mut self, a_key : &String, b_key : &String) -> bool {
        if a_key != b_key {
            if let Some(a) = self.vertices.get(a_key) {
                if let Some(b) = self.vertices.get(b_key) {
                    return Vertex::add_neighbor(a, b)
                }
            }
        }
        false
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    fn get_all_neighbors(&self, vertex: String) -> HashSet<String> {
        // Returns names of all neighbors of a node
        let mut neighbors: HashSet<String> = HashSet::new();
        if let Some(vc) = self.vertices.get(&vertex) {
            let ref adj = vc.ptr.borrow().adj;
            for vc in adj {
                neighbors.insert(vc.ptr.borrow().key.clone());
            }
        }
        neighbors
    }

    pub fn find_path(&self, src: String, dst: String) -> Vec<String> {
        // Returns a Vector of names of vertices in the path from source to destination
        // Uses simple BFS-based alogorithm
        let mut path: HashMap<String, String> = HashMap::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut return_path: Vec<String> = Vec::new();
        let mut path_exists = false;

        if self.vertices.contains_key(&src) && self.vertices.contains_key(&dst){
            let mut cur_node: String;
            let mut visited: HashSet<String> = HashSet::new();;

            queue.push_back(src.to_owned());
            visited.insert(src.clone());

            while queue.len() > 0 {
                if let Some(cur_node) = queue.pop_front() {
                    if cur_node == dst {
                        path_exists = true;
                        break;
                    }
                    let neighbors = self.get_all_neighbors(cur_node.clone());
                    for n in neighbors {
                        if !visited.contains(&n) {
                            visited.insert(n.clone());
                            queue.push_back(n.clone());
                            path.insert(n.clone(), cur_node.clone());
                        }
                    }
                }
            }

            // Going through the path HashMap to form the vector of Strings
            cur_node = dst.clone();
            if path_exists {
                while cur_node != src {
                    return_path.push(cur_node.to_owned());
                    if let Some(v) = path.get(&cur_node) {
                        cur_node = v.to_owned();
                    }
                }
                return_path.push(cur_node.to_owned());
            }
        }
        if return_path.len() == 1 {
            return Vec::new();
        } else {
            return_path.reverse();
            return return_path;
        }
    }
}

type VertexCellT = Rc<RefCell<Vertex>>;


#[derive(Eq)]
pub struct Vertex {
    adj: HashSet<VertexCell>,
    key: String,
}

#[derive(Eq)]
pub struct VertexCell{
    ptr : VertexCellT
}

impl Clone for VertexCell {
    fn clone(&self) -> Self {
        VertexCell {
            ptr : self.ptr.clone()
        }
    }

    fn clone_from(&mut self, source : &Self) {
        self.ptr = source.ptr.clone();
    }
}

impl PartialEq for VertexCell {
    fn eq(&self, other: &VertexCell) -> bool {
        (self.ptr.borrow().key == other.ptr.borrow().key)
    }

    fn ne(&self, other: &VertexCell) -> bool {
        !self.eq(other)
    }
}

impl Hash for VertexCell {
    fn hash<H :Hasher>(&self, state : &mut H) {
        let v = self.ptr.borrow();
        v.hash(state);
    }
}


impl Vertex {
    pub fn new(key: String) -> Vertex {
        Vertex {
            adj: HashSet::new(),
            key: key,
        }
    }

    pub fn new_cell(key: String) -> VertexCell {
        VertexCell {
            ptr : Rc::new(RefCell::new(Vertex::new(key)))
        }
    }

    pub fn add_neighbor(a: &VertexCell, b : &VertexCell) -> bool {
        let i = Vertex::add_link(a, b);
        Vertex::add_link(b, a) && i
    }

    fn add_link(a : &VertexCell, b : &VertexCell) -> bool{
        let mut mut_ref = a.ptr.borrow_mut();
        mut_ref.adj.insert(b.clone())
    }

    pub fn are_neighbors(a: &VertexCell, b : &VertexCell) -> bool {
        a.ptr.borrow().adj.contains(b)
    }

    pub fn get_name(self) -> String {
        return self.key;
    }
}

impl Clone for Vertex {
    fn clone(&self) -> Self {
        Vertex {
            key : self.key.clone(),
            adj : self.adj.clone(),
        }
    }

    fn clone_from(&mut self, source : &Self) {
        self.key = source.key.clone();
        self.adj = source.adj.clone();
    }
}


impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        (self.key == other.key)
    }

    fn ne(&self, other: &Vertex) -> bool {
        (self.key != other.key)
    }
}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}



#[cfg(test)]
mod graph_tests {
    pub use super::{Vertex, Graph};

    #[test]
    // lets make sure vertices can get added
    fn graph_test_add_vertex() {
        let mut g1 = Graph::new();
        let mut g2 = Graph::new();
        
        g1.add_vertex("a".to_string());
        g2.add_vertices(vec!["a".to_string(), "b".to_string()].to_owned());
        assert_eq!(g1.len(), 1);
        assert_eq!(g2.len(), 2);
    }

    #[test]
    // a simple find path test
    fn graph_test_find_path() {
        let mut g = Graph::new();
        g.add_vertices(vec!["a".to_string(), "b".to_string()].to_owned());
        assert_eq!(g.len(), 2);

        g.add_edge(&"a".to_string(), &"b".to_string());
        let path = g.find_path("a".to_string(), "b".to_string());
        assert!(path.len() > 0);
    }

    #[test]
    // more complex find path test
    fn graph_test_find_path_2() {
        let mut g = Graph::new();
        g.add_vertices(vec!["a".to_string(), "b".to_string(), "c".to_string(),
        "d".to_string()].to_owned());

        assert_eq!(g.len(), 4);
        g.add_edge(&"a".to_string(), &"b".to_string());
        g.add_edge(&"c".to_string(), &"d".to_string());
        g.add_edge(&"a".to_string(), &"d".to_string());
        let path = g.find_path("c".to_string(), "b".to_string());
        assert_eq!(path.len(), 4);
    }


    #[test]
    // test the case where there is no graph
    fn graph_test_no_path() {
        let mut g = Graph::new();
        g.add_vertices(vec!["a".to_string(), "b".to_string(), "c".to_string(),
        "d".to_string()].to_owned());

        assert_eq!(g.len(), 4);
        g.add_edge(&"a".to_string(), &"b".to_string());
        g.add_edge(&"c".to_string(), &"d".to_string());
        let path = g.find_path("c".to_string(), "b".to_string());
        assert_eq!(path.len(),0);
    }
}

#[cfg(test)]
mod vertex_tests {
    use super::{Vertex};

    #[test]
    fn two_nodes() {
        let cell1 = Vertex::new_cell("one".to_string());
        let cell2 = Vertex::new_cell("two".to_string());

        Vertex::add_neighbor(&cell1, &cell2);

        assert!(cell1.ptr.borrow().adj.len() == cell2.ptr.borrow().adj.len() && cell1.ptr.borrow().adj.len() == 1);
    }

    #[test]
    fn three_nodes() {
        let cell1 = Vertex::new_cell("one".to_string());
        let cell2 = Vertex::new_cell("two".to_string());
        let cell3 = Vertex::new_cell("three".to_string());

        Vertex::add_neighbor(&cell1, &cell2);
        Vertex::add_neighbor(&cell1, &cell3);

        let mut b : bool = cell1.ptr.borrow().adj.len() == 2;
        b = b && (cell2.ptr.borrow().adj.len() == 1);
        b = b && (cell3.ptr.borrow().adj.len() == 1);

        assert!(b);
    }

    #[test]
    fn two_nodes_mutate() {
        let cell1 = Vertex::new_cell("one".to_string());
        let cell2 = Vertex::new_cell("two".to_string());

        Vertex::add_neighbor(&cell1, &cell2);
        {
            cell2.ptr.borrow_mut().key = "changed".to_string();
        }

    }
}
