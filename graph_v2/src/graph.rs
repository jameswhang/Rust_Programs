#[doc="
    Exposes only the Graph struct. Most of logic can be found in the Vertex struct. Most of the logic
    is found in the Vertex Struct

    Comments:
        - By not exposing Vertex, that means we have to operate through the use of keys. This causes
        non-idomatic code. We chose this so that there would be no illegal mutation such as changing
        keys by assignment, therefore ruining the HashMap
        - Chose to do Rc and RefCell because we wanted to do some cool Drop behaviors, but it didn't
        pan out, but it works fine.
        - VertexCell struct made to wrap VertexCellT to implement Traits

    Assumptions:
        - Vertex Keys are strings
"]
use std::collections::{HashSet, HashMap, VecDeque};
use std::cell::{RefCell};
use std::rc::Rc;
use std::hash::{Hash, Hasher};


/// Public container for creating graph structure that is mutable by reference
/// A hashmap by String was chosen since vertex keys are strings.
/// Side-effect is that once a graph instance goes out of scope, then all the nodes
/// are out of scope as well, which is proper behavior.
#[derive(Eq)]
pub struct Graph {
    vertices: HashMap<String, VertexCell>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: HashMap::new(),
        }
    }

    /// Adds a single vertex. Ensures only adding vertices with unique keys.
    /// @param key : String - string to be used as key in vertex
    pub fn add_vertex(&mut self, key : String) {
        self.vertices.entry(key.clone()).or_insert(Vertex::new_cell(key));
    }

    /// Adds vertices. Ensures only adding vertices with unique keys.
    /// @param keys : Vec<String> - string to be used as key in vertex
    pub fn add_vertices(&mut self, keys: Vec<String>) {
        for key in keys {
            self.add_vertex(key);
        }
    }

    /// Adds vertices. Ensures only simple graphs i.e. no self-loopoing
    /// @param a_key : String - key of node in graph
    /// @param b_key : String - key of node in graph
    ///
    /// @return bool - returns whether
    pub fn add_edge(&mut self, a_key : &String, b_key : &String) -> bool {
        //prevents self-loop
        if a_key != b_key {
            if let Some(a) = self.vertices.get(a_key) {
                if let Some(b) = self.vertices.get(b_key) {
                    return Vertex::add_neighbor(a, b)
                }
            }
        }

        false
    }

    /// Returns number of nodes in graph. Uses len hashmap
    /// @return usize -  number of nodes in graph
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// Returns keys of all neighbors of a node
    /// @param vertex_key : String - key of node to find neighbors of
    ///
    /// @return HashSet<String> -
    fn get_all_neighbors(&self, vertex_key: String) -> HashSet<String> {
        let mut neighbors: HashSet<String> = HashSet::new();

        if let Some(vc) = self.vertices.get(&vertex_key) {
            let ref adj = vc.ptr.borrow().adj;

            for vc in adj {
                neighbors.insert(vc.ptr.borrow().key.clone());
            }
        }

        neighbors
    }

    // Returns a Vector of names of vertices in the path from source to destination
    // Uses simple BFS-based alogorithm
    /// @param src_key : String - key of node in graph
    /// @param dst_key : String - key of node in graph
    ///
    /// @return Vec<String>
    pub fn find_path(&self, src_key: String, dst_key: String) -> Vec<String> {
        let mut path: HashMap<String, String> = HashMap::new();
        //stack for BFS
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut return_path: Vec<String> = Vec::new();
        let mut path_exists = false;

        if self.vertices.contains_key(&src_key) && self.vertices.contains_key(&dst_key) {
            let mut cur_node: String;
            let mut visited: HashSet<String> = HashSet::new();;

            queue.push_back(src_key.to_owned());
            visited.insert(src_key.clone());

            while queue.len() > 0 {
                if let Some(cur_node) = queue.pop_front() {
                    if cur_node == dst_key {
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
            cur_node = dst_key.clone();
            if path_exists {
                while cur_node != src_key {
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


impl PartialEq for Graph {
    fn eq(&self, other: &Graph) -> bool {
        (self.vertices == other.vertices)
    }

    fn ne(&self, other: &Graph) -> bool {
        !(self.eq(other))
    }
}




/// See comments for choice in using this type
type VertexCellT = Rc<RefCell<Vertex>>;


#[derive(Eq)]
struct Vertex {
    adj: HashSet<VertexCell>,
    key: String,
}


#[derive(Eq)]
struct VertexCell{
    ptr : VertexCellT
}


/// Following Traits implemented to use with HashSet
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
    /// Creates a single vertex.
    /// @param key : String - string to be used as key in vertex
    ///
    /// @return Vertex
    pub fn new(key: String) -> Vertex {
        Vertex {
            adj: HashSet::new(),
            key: key,
        }
    }

    /// Helper to create a single vertexcell.
    /// @param key : String - string to be used as key in vertex
    ///
    /// @return VertexCell
    pub fn new_cell(key: String) -> VertexCell {
        VertexCell {
            ptr : Rc::new(RefCell::new(Vertex::new(key)))
        }
    }

    /// Makes two vertices neighbors. Ensures undirected edge
    /// @param a : &VertexCell
    /// @param b : &VertexCell
    ///
    /// @return bool - whether a brand new edge was added
    pub fn add_neighbor(a: &VertexCell, b : &VertexCell) -> bool {
        let i = Vertex::add_link(a, b);
        Vertex::add_link(b, a) && i
    }

    /// Creates a directed edge between vertices
    /// @param a : &VertexCell
    /// @param b : &VertexCell
    ///
    /// @return bool - whether a brand new edge was added
    fn add_link(a : &VertexCell, b : &VertexCell) -> bool{
        let mut mut_ref = a.ptr.borrow_mut();
        mut_ref.adj.insert(b.clone())
    }


    /// Checks whether two vertices are neighbors
    /// @param a : &VertexCell
    /// @param b : &VertexCell
    ///
    /// @return bool - whether a brand new edge was added
    pub fn are_neighbors(a: &VertexCell, b : &VertexCell) -> bool {
        a.ptr.borrow().adj.contains(b)
    }

    pub fn get_name(self) -> String {
        return self.key;
    }
}


/// Following Traits implemented to use with HashSet
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


/*********************** TESTS **************************************************/


#[cfg(test)]
mod graph_tests {
    use super::{Graph};
    use std::collections::{HashSet};

    #[test]
    fn graph_test_add_vertex() {
        let mut g1 = Graph::new();

        g1.add_vertex("a".to_string());
        assert_eq!(g1.len(), 1);
    }

    #[test]
    fn graph_test_add_two_vertices() {
        let mut g1 = Graph::new();

        g1.add_vertex("a".to_string());
        g1.add_vertex("b".to_string());
        assert_eq!(g1.len(), 2);
    }

    #[test]
    fn graph_test_add_vertices() {
        let mut g2 = Graph::new();

        g2.add_vertices(vec!["a".to_string(), "b".to_string()].to_owned());
        assert_eq!(g2.len(), 2);
    }


    #[test]
    fn graph_test_get_neighbors() {
        let mut g = Graph::new();
        g.add_vertices(vec!["a".to_string(), "b".to_string(), 'c'.to_string()].to_owned());
        g.add_edge(&"a".to_string(), &"b".to_string());
        let results : HashSet<String> = g.get_all_neighbors("a".to_string());

        let mut expected : HashSet<String> = HashSet::new();
        expected.insert("b".to_string());

        assert_eq!(results, expected);
    }


    #[test]
    fn graph_test_get_neighbors_more() {
        let mut g = Graph::new();
        g.add_vertices(vec!["a".to_string(), "b".to_string(), 'c'.to_string()].to_owned());
        g.add_edge(&"a".to_string(), &"b".to_string());
        g.add_edge(&"a".to_string(), &"c".to_string());
        let results : HashSet<String> = g.get_all_neighbors("a".to_string());

        let mut expected : HashSet<String> = HashSet::new();
        expected.insert("c".to_string());
        expected.insert("b".to_string());

        assert_eq!(results, expected);
    }


    // a simple find path test
    #[test]
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
