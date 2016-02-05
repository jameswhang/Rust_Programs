#[doc="
    Authors: Adel and James, Whang and Lahlou.
    NetIDs: syw973, adl538

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

extern crate graph_v2;
use std::io::{BufRead, BufReader, Read, stdin};
use std::fs::{File};
use std::path::Path;
use graph_v2::graph::{Graph};



fn main() {
    let inputfile = process_commandline();
    let graph = process_file(inputfile);
    handle_queries(stdin(), &graph);
}

fn process_file<R : Read>(input: R) -> Graph {
    let mut graph : Graph = Graph::new();
    let mut previous : Option<String> = None;
    let reader = BufReader::new(input);

    for line in reader.lines() {
        //iterate through tokens seperated by whitespace aka token names
        for node in line.unwrap().split_whitespace() {
            graph.add_vertex(node.to_string());
            if let Some(pnode) = previous {
                graph.add_edge(&pnode, &node.to_string());
            }

            previous = Some(node.to_string());
        }

        previous = None;
    }

    graph
}


fn handle_queries<R: Read>(input : R, graph : &Graph) {
    let mut previous  : Option<String> = None;
    let reader = BufReader::new(input);

    for line in reader.lines() {
        for node in line.unwrap().split_whitespace() {
            if let Some(pnode) = previous {
                let path = graph.find_path(pnode.clone(), node.to_string());
                if path.len() == 0 {
                    println!("There is no path from {} to {}", pnode.clone(), node.to_string());
                } else {
                    let mut path_string = "".to_string();
                    for node in path {
                        path_string.push_str(&node);
                        path_string.push_str(" ");
                    }
                    println!("{}", path_string);
                }
            }

            previous = Some(node.to_string());
        }

        previous = None;
    }
}


fn process_commandline() -> BufReader<File> {
    let args : Vec<_>= std::env::args().collect();

    if args.len() != 2 {
        panic!("usage: cargo run graph.dat");
    }

    let mut filename = args[1].clone();
    let base_path = Path::new(&args[0]).parent().unwrap();
    filename = fix_path(base_path, &filename);

    BufReader::new(open_file(&filename))
}

fn open_file(filename: &String) -> File{
    if let Ok(file) = File::open(&filename) {
        file
    } else {
        panic!("Couldn't open file");
    }
}

///currently used to solve issue of files not being called relative to the execution directory
fn fix_path(base_path : &Path, filename : &String) -> String {
	base_path.join(Path::new(&filename)).to_str().unwrap().to_owned()
}

#[cfg(test)]
mod graph_usage_tests{
    mod graph_building_test{

    }


    mod graph_querying_test {

    }
}
