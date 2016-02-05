use std::collections::{HashSet, HashMap};
use std::cell::{RefCell};
use std::rc::Rc;
use std::hash::{Hash, SipHasher, Hasher};
use std::marker::PhantomData;


#[derive(Debug)]
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

    pub fn add_edge(&mut self, a_key : String, b_key : String) -> bool {
        if let Some(a) = self.vertices.get(&a_key) {
            if let Some(b) = self.vertices.get(&b_key) {
                return Vertex::add_neighbor(a, b)
            }
        }

        false
    }

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


#[derive(Eq, Debug)]
pub struct Vertex {
    adj: HashSet<VertexCell>,
    key: String,
}

#[derive(Debug, Eq)]
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
        }git
    }
}
