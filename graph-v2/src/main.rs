extern crate graph;

use graph::{Graph, Vertex};

fn main() {
//    let inputfile = process_commandline();
    let graph = process_file(inputfile);
}

fn process_file<R: Read>(input :R ) -> Graph<String> {
    let mut graph: Graph = Graph::new();
    let mut previous = None; 
    let reader = BufReader::new(input);

    for line in reader.lines() {
        let tokens = split_whitespace(&line.unwrap());

        for node in tokens {
            graph.add_vertex(node);

            if let Some(pnode) = previous {
                graph.add_edge(pnode, node);
            }

            previous = Some(node);
        }
    }
    graph
}
