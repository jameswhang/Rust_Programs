# Correct


## Description
The purpose of correct is to find possible corrections for misspelled words. It consists of two phases: The first phase is a training module, which consumes a corpus of correctly spelled words and counts the number of occurrences of each word. The second phase uses the results of the first to check individual words. Specifically, it checks whether each word is spelled correctly according to the training module and, if not, whether “small edits” can reach a variant that is correctly spelled.


Basically a Rust implementation of <a href="http://norvig.com/spell-correct.html">Peter Norvig's Idea</a>

## Usage
git clone https://github.com/jameswhang/Rust_Programs.git
cd correct
cargo run [train.txt] < [input.txt]
