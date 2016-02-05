#[doc="
    Authors: Adel and James, Whang and Lahlou.

    This program uses the included graph data structure to


    Comments:

    Assumptions:
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
            println!("Adding {}", node);
            if let Some(pnode) = previous {
                graph.add_edge(&pnode, &node.to_string());
                println!("Edge from {} to {}", pnode, node);
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
        let mut path_string = String::new();

        for node in line.unwrap().split_whitespace() {
            if let Some(pnode) = previous {
                if let Some(vertex_keys) = graph.search_path(pnode, node.to_string()) {
                    path_string.push_str(&vertex_keys.join(" "));
                }
            }

            previous = Some(node.to_string());
        }
    }
}


fn process_commandline() -> BufReader<File> {
    let args : Vec<_>= std::env::args().collect();

    if args.len() != 2 {
        panic!("usage: cargo run graph_file.dat");
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
