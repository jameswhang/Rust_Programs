#[doc="
    @authors: Adel and James, Whang and Lahlou.

    @description:

    @comments:

    @assumptions:
"]

extern crate graph;
extern crate regex;
use std::io::{BufRead, BufReader, Read, stdin};
use std::fs::{File};
use regex::{Regex};
use graph::{Graph, Vertex};


const SPLIT_WHITESPACE : Regex = Regex::new(r"\w+").unwrap();


fn main() {
    let inputfile = process_commandline();
    let graph = process_file(inputfile);
    handle_queries(stdin(), &graph);
}

fn process_file<R : Read>(input: R) -> Graph<String>{
    let mut graph : Graph<String> = Graph::new();
    let mut previous : Option<String> = None;
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


fn handle_queries<R: Read>(input : R, graph : &Graph<String>) {
    let mut previous  : Option<String> = None;
    let reader = BufReader::new(input);

    for line in reader.lines() {
        let mut path_string = String::new();
        let tokens = split_whitespace(&line.unwrap());

        for node in tokens {
            if let Some(pnode) = previous {
                if let Some(sub_path_string) = Graph::search_path(pnode, node) {
                    path_string.push_str(&sub_path_string);
                }
            }

            previous = Some(node);
        }
    }
}

fn split_whitespace(line : &String) -> Vec<String> {
    line.split(&SPLIT_WHITESPACE).collect::<Vec<_>>()
}

fn process_commandline() -> BufReader<File> {
    let args : Vec<_>= std::env::args().collect();

    if args.len() != 2 {
        panic!("usage: cargo run graph_file.dat");
    }

    let filename = args[1];
    BufReader::new(open_file(filename));
}

fn open_file(filename: &String, panic_message : &str) -> File{
    if let Ok(file) = File::open(filename) {
        file
    } else {
        panic!(panic_message);
    }
}


#[cfg(test)]
mod graph_usage_tests{
    mod graph_building_test{

    }


    mod graph_querying_test {

    }
}
