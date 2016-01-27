use std::io::{BufRead,BufReader,Read,stdin};
use std::env;
use std::fs::File;

#[doc="
    Takes in a training file as argument, and counts the frequencies of the words in the training
    file.
    Then takes in inputs from stdin, and finds the 'nearest' correction to each word and prints it. 
    If the word already seems to be a correct word, it won't correct it. 
    If the word doesn't have any corrections available, '-' will be printed.

Author: James Whang (syw973, sungyoonwhang2017@u.northwestern.edu)

Assumptions:
    Training file doesn't have typos
    One word per line in the input text to be corrected
"]

type CountTable = std::collections::HashMap<String, usize>;
type Candidates = std::vec::Vec<String>;
type Edits = std::collections::HashSet<String>;
static ALPHABETS : &'static str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./correct [train file] ")
    }
    let f = File::open(&args[1]).expect("Error opening training file ");
    let table = form_table(f); 

    correct(stdin(), table);
}

fn correct<R: Read>(reader: R, table: CountTable) {
    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(word)) = lines.next() {
        if table.contains_key(&word) { // If the word is already known, no need to go through this 
            println!("{}", word);
            continue;
        }

        let edit_twos = edits_two(&word);

        let mut closest = "-".to_owned(); // String to hold the closest word.
        let mut closest_count = 0; // If count gets left as 0, there is no word "close enough" to
                                    // the given test word
        
        for edit_two_word in edit_twos.iter() {
            if table.contains_key(edit_two_word) {
                if table[edit_two_word] > closest_count {
                    closest = edit_two_word.to_owned();
                    closest_count = table[edit_two_word];
                }
            }
        }
        println!("{}, {}", word, closest);
    }
}

fn edits_two(word: &String) -> Edits {
    let mut edits = Edits::new();
    let edit_ones = edits_one(&word);

    for edit_one_word in edit_ones.iter() {
        edits = edits.union(&edits_one(&edit_one_word.to_owned())).cloned().collect();
    }
    edits
}

fn edits_one(word: &String) -> Edits {
    let deletes = find_deletions(word.to_owned());
    let replaces = find_replacements(word.to_owned());
    let inserts = find_insertions(word.to_owned());
    let trans = find_transpositions(word.to_owned());

    let mut edits = Edits::new();

    edits = edits.union(&deletes).cloned().collect();
    edits = edits.union(&replaces).cloned().collect();
    edits = edits.union(&inserts).cloned().collect();
    edits = edits.union(&trans).cloned().collect();
    edits
}

fn find_deletions(word: String) -> Edits {
    let mut edits = Edits::new();
    let mut deleted: String;

    if word.len() <= 1 {
        return edits
    }

    for i in 0..word.len() as usize {
        if i == 0 {
            deleted = (&word[i + (1 as usize)..]).to_string();
        } else if i == word.len() - 1 {
            deleted = (&word[..i]).to_string();
        } else {
            deleted = (&word[..i]).to_string();
            deleted = deleted + &word[i + (1 as usize)..];
        }
        edits.insert(deleted.clone());
    }
    edits
}


fn find_replacements(word: String) -> Edits {
    let mut edits = Edits::new(); 
    let mut replaced : String;

    for i in 0..word.len() as usize {
        for c in ALPHABETS.chars() {
            replaced = (&word[..i]).to_string();
            replaced.push(c);
            replaced = replaced + &word[i + (1 as usize)..];
            edits.insert(replaced.clone());
        }
    }
    edits
}

fn find_insertions(word: String) -> Edits {
    let mut edits = Edits::new();
    let mut inserted : String;

    for i in 0..word.len() + 1 as usize {
        for c in ALPHABETS.chars() {
            inserted = (&word[..i]).to_string();
            inserted.push(c);
            inserted = inserted + &word[i..];
            edits.insert(inserted.clone());
        }
    }
    edits
}

fn find_transpositions(word: String) -> Edits {
    let mut edits = Edits::new();
    let mut transposed: String;
    let mut characters: Vec<char> = vec![];

    for c in word.chars() {
        characters.push(c);
    }

    for i in 0..word.len() - 1 as usize {
        transposed = (&word[..i]).to_string();
        transposed.push(characters[i + (1 as usize)]);
        transposed.push(characters[i]);
        transposed = transposed + &word[i + (2 as usize)..];
        edits.insert(transposed.clone());
    }
    edits
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


fn form_table<R: Read>(reader: R) -> CountTable {
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


#[allow(dead_code)]
fn increment_word(map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
}

#[cfg(test)]
mod form_table_tests {
    use super::{CountTable, form_table};
    use std::io::{Read, Result};

    #[test]
    fn form_table_test_basic_1() {
        let table = make_test_table("Hello, world!");
        assert_saved(&table, "world", 1);
        assert_saved(&table, "hello", 1);
        assert_none(&table, "hellooo");
    }

    #[test]
    fn form_table_test_basic_2() {
        let table = make_test_table("Of the dogs, By the dogs, For the dogs");
        assert_saved(&table, "dogs", 3);
        assert_saved(&table, "the", 3);
        assert_saved(&table, "of", 1);
        assert_none(&table, "people");
    }

    #[test]
    fn form_table_test_empty() {
        let table = make_test_table("");
        assert_none(&table, "Hi");
    }

    fn make_test_table(input: &str) -> CountTable {
        let mock_read = StringReader::new(input.to_owned());
        form_table(mock_read)
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

#[cfg(test)]
mod edits_test {
    use super::{find_transpositions, find_deletions, find_replacements, find_insertions, edits_one, edits_two};

    #[test]
    fn find_transpositions_test() {
        let trans_1 = find_transpositions("ab".to_owned());
        assert!(trans_1.contains("ba"));

        let trans_2 = find_transpositions("abc".to_owned());
        assert!(trans_2.contains("bac"));
        assert!(trans_2.contains("acb"));
        assert!(!trans_2.contains("abcd"));
    }

    #[test]
    fn find_deletions_test() {
        let dels = find_deletions("ab".to_owned());
        assert!(dels.contains("b"));
        assert!(dels.contains("a"));
        assert!(!dels.contains("ab"));

        let dels2 = find_deletions("abcd".to_owned());
        assert!(dels2.contains("abc"));
        assert!(dels2.contains("bcd"));
        assert!(dels2.contains("acd"));
        assert!(dels2.contains("abd"));

    }

    #[test]
    fn find_replacements_test() {
        let reps = find_replacements("abc".to_owned());
        assert!(reps.contains("abd"));
        assert!(reps.contains("dbc"));
        assert!(reps.contains("acc"));

        assert!(!reps.contains("accd"));
        assert!(!reps.contains("cd"));
        assert!(!reps.contains("bcd"));
        assert!(!reps.contains("acd"));
    }


    #[test]
    fn find_insertions_test() {
        let ins = find_insertions("ab".to_owned());
        assert!(ins.contains("abc"));
        assert!(ins.contains("abb"));
        assert!(ins.contains("abd"));

        assert!(!ins.contains("acc"));
        assert!(!ins.contains("abcd"));
    }

    #[test]
    fn edit1_test() {
        let edit1s = edits_one(&"abcd".to_owned());
        assert!(edit1s.contains("abd"));
        assert!(edit1s.contains("abcde"));
        assert!(edit1s.contains("abcz"));
        assert!(edit1s.contains("bacd"));

        assert!(!edit1s.contains("adzcd"));
        assert!(!edit1s.contains("aabed"));
        assert!(!edit1s.contains("cccd"));
        assert!(!edit1s.contains("cd"));
    }

    #[test]
    fn edit2_test() {
        let edit2s = edits_two(&"abcd".to_owned());
        assert!(edit2s.contains("aabcd"));
        assert!(edit2s.contains("aabed"));
        assert!(edit2s.contains("badc"));
        assert!(edit2s.contains("abcdef"));
        assert!(edit2s.contains("cd"));

        assert!(!edit2s.contains("d"));
        assert!(!edit2s.contains("abcdefg"));
        assert!(!edit2s.contains("cqfg"));
    }
}
