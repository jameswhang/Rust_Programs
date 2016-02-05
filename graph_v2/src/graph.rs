#[doc="
    This is an implementation of Graph that allows the addition of vertices and edges by supplying
    a key (String). The keys must be unique and so there are no nodes with duplicate key values.

    The implementation was decided upon to maximize learning. We could've done a simple adjancency matrix
    or adjacency, or even a HashMap<String, Vec<String>> since the homework didn't say the nodes needed to be mutable.
    Instead, we wrote a few different implementations, some that relied on heavy use of life times,
    generics (which needed PhatomData markers). Ultimately, we went with a representation to get
    practice using reference types other than box, specifically RefCell and Rc. We also practiced
    using Traits.

    Otherwise, we made choices related to performance. HashMap allows O(1) access to any node by unique
    hashed key value. HashSets for adjancency prohibits duplicates (though apparently is super unstable).
    While slightly more expensive operation wise, space wise, using HashSets is similar to adjacency lists.
    The VertexCell struct was made to wrap our reference for convenience, so that we could implementations
    traits.

    Comments:
        - Structs VertexCell and Vertex were exposed through pub to do tests
        - Structs don't derive debug because they cause an infinite print loops and overrun the stack

    Assumptions:
        - Simple graph aka nodes can't share edges with themself
        - Keys are only Strings
        - We want only unique keys
        - We want any path from origin to destination vertex
        - Graph is undirected
        - Graph vertices must be mutable
"]

use std::collections::{HashSet, HashMap};
use std::cell::{RefCell};
use std::rc::Rc;
use std::hash::{Hash, SipHasher, Hasher};
use std::marker::PhantomData;


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

    pub fn search_path(&self, from : String, to : String) -> Option<Vec<String>> {
        unimplemented!();
    }

    pub fn find_path_strings(&self, from : String, to : String) -> Vec<Vertex> {
        // if let Some(from_rc) = self.get(from) {
        //     if let Some(to_rc) = self.get(to) {
        //         if let Ok(from_cell) = from_rc.try_unwrap() {
        //             if let Ok(to_cell) = to_rc.try_unwrap() {
        //
        //             }
        //         }
        //     }
        // }

        panic!("Vertex with that key not in graph");
    }

    // pub fn find_path

    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}

//
// type VertexPtr<'a, T> = Option<Box<Vertex<'a, T>>>;
//
// struct Vertex <'a, T> where T : 'a {
//     key: T,
//     neighbors : HashSet<VertexPtr<'a, T>>,
//     phantom: PhantomData<&'a T>,
// }


type VertexCell_t = Rc<RefCell<Vertex>>;


#[derive(Eq)]
pub struct Vertex {
    adj: HashSet<VertexCell>,
    key: String,
}

#[derive(Eq)]
pub struct VertexCell{
    ptr : VertexCell_t
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


impl Vertex{
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
        let mut i = Vertex::add_link(a, b);
        Vertex::add_link(b, a) && i
    }

    fn add_link(a : &VertexCell, b : &VertexCell) -> bool{
        let mut mut_ref = a.ptr.borrow_mut();
        mut_ref.adj.insert(b.clone())
    }

    pub fn are_neighbors(a: &VertexCell, b : &VertexCell) -> bool {
        a.ptr.borrow().adj.contains(b)
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

            let mut b : bool = (cell1.ptr.borrow().adj.len() == 2);
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


            // if let Some(c2) = cell1.ptr.borrow().adj.iter().next() {
            //     assert_eq!( c2.ptr.borrow().key.clone(), cell2.ptr.borrow().key);
            // }
        }
    }


    mod graph_tests {
        use super::{Vertex, Graph};

        #[test]
        fn insert_node(){
            let mut g = Graph::new();

            g.add_vertex("v1".to_string());
            assert_eq!(g.len(), 1);
        }
    }
}
