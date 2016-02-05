extern crate graph_v2;
extern crate regex;

use graph_v2::{adelgraph};
use regex::{Regex};
use std::io;
use std::io::{BufRead, BufReader, Read, stdin};
use std::fs::{File};

/*
const SPLIT_WHITESPACE : Regex = Regex::new(r"\w+").unwrap();
*/

fn main() {
//    let inputfile = process_commandline();
//    let graph = process_file(inputfile);
}

fn process_file<R: Read>(input :R) {
    unimplemented!();
/*
    let mut graph: graph::Graph = graph::Graph::new();
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
    */
}

/*
fn split_whitespace(line: &String) -> Vec<String> {
    line.split(&SPLIT_WHITESPACE).collect::<Vec<_>>()
}
*/
