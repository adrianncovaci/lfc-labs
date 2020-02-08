use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::any::type_name;

fn print_type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

struct State {
    name: String,
    directions: HashMap<String, String>,
    end_state: bool,
}

impl State {
    fn new() -> State {
        State {
            name: String::new(),
            directions: HashMap::new(),
            end_state: false,
        }
    }
}

fn read_lines(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        println!("{}", print_type_of(line));
    }
}

fn main() {
    let filename = "gramm.txt";
    read_lines(filename.to_string());
}