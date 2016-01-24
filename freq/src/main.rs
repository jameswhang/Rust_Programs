use std::io::{BufRead,BufReader,Read,stdin};

#[doc="
Counts the frequencies of words read from the standard input, and print
a sorted frequency table.

Author: James Whang (syw973, sungyoonwhang2017@u.northwestern.edu)

Assumptions:
    - Punctuations ( , | . | ! | ? ) are stripped
        ex) 'hello, world' gives the same result as 'hello world!'
    - All words are lowercased
    - Quotations will be also stripped
    - Words containing quotes will be regarded as a single word.
        ex1) I'm != I m
        ex2) I'm == Im 
    - Dashes won't be stripped (-, --, whatever)
"]
fn main() {
    produce_output(&read_input(stdin()));
}

fn parse_input(input: String) -> String {
    // These should get deleted from original string
    let chars_to_trim = &["!", "?", "\"", "'"]; 
    // These should simply be replaced by space, otherwise "hello,world" becomes "helloworld"
    let chars_to_replace = &[",", "."]; 
    let mut result = input.to_lowercase();
    for character in chars_to_trim {
        result = result.replace(character, "");
    }
    for character in chars_to_replace {
        result = result.replace(character, " ");
    }
    result
}


fn read_input<R: Read>(reader: R) -> CountTable {
    let mut lines = BufReader::new(reader).lines();
    let mut table = CountTable::new();

    while let Some(Ok(line)) = lines.next() {
        let replaced_line = parse_input(line.to_owned());
        let words: Vec<&str> = replaced_line.split_whitespace().collect();
        for word in words {
            increment_word(&mut table, word.to_owned());
        }
    }
    return table
}

type CountTable = std::collections::HashMap<String, usize>;

#[allow(dead_code)]
fn increment_word(map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
}

#[cfg(test)]
mod read_input_tests {
    use super::{CountTable, read_input};
    use std::io::{Read, Result};

    #[test]
    fn read_input_test_basic_1() {
        let table = form_table("Hello, world!");
        assert_saved(&table, "world", 1);
        assert_saved(&table, "hello", 1);
        assert_none(&table, "hellooo");
    }

    #[test]
    fn read_input_test_basic_2() {
        let table = form_table("Of the dogs, By the dogs, For the dogs");
        assert_saved(&table, "dogs", 3);
        assert_saved(&table, "the", 3);
        assert_saved(&table, "of", 1);
        assert_none(&table, "people");
    }

    #[test]
    fn read_input_test_empty() {
        let table = form_table("");
        assert_none(&table, "Hi");
    }

    fn form_table(input: &str) -> CountTable {
        let mock_read = StringReader::new(input.to_owned());
        read_input(mock_read)
    }

    fn assert_saved(table: &CountTable, test: &str, expected_num: usize) {
        assert_eq!(Some(&expected_num), table.get(test));
    }

    fn assert_none(table: &CountTable, test: &str) {
        assert_eq!(None, table.get(test));
    }

    struct StringReader {
        contents: Vec<u8>,
        position: usize,
    }

    impl StringReader {
        fn new(s: String) -> Self {
            StringReader {
                contents: s.into_bytes(),
                position: 0,
            }
        }
    }

    impl Read for StringReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut count = 0;

            while self.position < self.contents.len() && count < buf.len() {
                buf[count] = self.contents[self.position];
                count += 1;
                self.position += 1;
            }

            return Ok(count);
        }
    }
}

#[cfg(test)]
mod parse_input_tests {
    use super::{parse_input};

    #[test]
    fn parse_input_strip_test() {
        assert_parse_eq("hello!", "hello");
        assert_parse_eq("hello!?!?!", "hello");
        assert_parse_eq("Hello\"", "hello");
    }

    #[test]
    fn parse_input_replace_test() {
        assert_parse_eq("hello,world", "hello world");
        assert_parse_eq("hello.world", "hello world");
    }

    #[test]
    fn parse_input_lower_test() {
        assert_parse_eq("HELLO", "hello");
        assert_parse_eq("hello", "hello");
        assert_parse_eq("HelLo", "hello");
    }

    fn assert_parse_eq(string: &str, result: &str) {
        assert_eq!(result.to_owned(), parse_input(string.to_owned()));
    }
}

#[cfg(test)]
mod increment_word_tests {
    use super::{increment_word, CountTable};

    #[test]
    fn inserts_if_empty() {
        let mut h = CountTable::new();
        increment_word(&mut h, "one".to_owned());

        assert_eq!(Some(&1), h.get("one"));
        assert_eq!(1, h.len());
    }

    #[test]
    fn increments_if_present() {
        let mut under_test = fixture();
        let mut expected   = fixture();

        increment_word(&mut under_test, "three".to_owned());
        expected.insert("three".to_owned(), 4);

        assert_eq!(expected, under_test);
    }

    #[test]
    fn insert_if_absent() {
        let mut under_test = fixture();
        let mut expected   = fixture();

        increment_word(&mut under_test, "one".to_owned());
        expected.insert("one".to_owned(), 1);

        assert_eq!(expected, under_test);
    }

    fn fixture() -> CountTable {
        let mut h = CountTable::new();
        h.insert("two".to_owned(), 2);
        h.insert("three".to_owned(), 3);

        assert_eq!(None, h.get("one"));
        assert_eq!(Some(&2), h.get("two"));
        assert_eq!(Some(&3), h.get("three"));
        assert_eq!(2, h.len());
        h
    }
}

fn produce_output(table: &CountTable) {
    for (word, count) in table {
        println!("{}: {}", word, count);
    }
}
