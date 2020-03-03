use std::fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(PartialEq, Eq, Clone)]
pub struct State {
    pub name: String,
    pub directions: HashMap<String, char>,
    pub start_state: bool,
    pub end_state: bool,
}

impl State {
    pub fn new(name: char) -> State {
        State {
            name: name.to_string(),
            directions: HashMap::new(),
            start_state: false,
            end_state: false,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone)]
pub struct StateList {
    pub list: Vec<State>,
    capacity: usize,
    pub values: Vec<char>,
}

impl StateList {
    pub fn new() -> StateList {
        StateList {
            list: Vec::new(),
            capacity: 0,
            values: Vec::new(),
        }
    }

    pub fn append_state(&mut self, new: State) {
        self.list.push(new);
        self.capacity += 1;
    }

    pub fn read_values(&mut self, val: &String) {
        for c in val.chars() {
            if c != ' ' {
                self.values.push(c);
            }
        }
    }

    pub fn get_state_by_name(&self, name: char) -> Option<usize> {
        if self.list.is_empty() {
            return None;
        }
        let mut curr: Option<usize> = Some(0);
        for (index, state) in self.list.iter().enumerate() {
            if state.name == name.to_string() {
                curr = Some(index);
            }
        }
        curr
    }

    pub fn has_state_by_name(&self, name: char) -> bool {
        for state in self.list.iter() {
            if state.name == name.to_string() {
                return true;
            }
        }
        return false;
    }

    pub fn read_directions(&mut self, filename: String) {
        let mut curr: Option<usize> = None;
        let mut dir: char = '`';
        let mut val: char = '`';
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
    
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            for (line_index, c) in line.chars().enumerate() {
                if index >= 2 && c != ' ' && c != '>' {
                    if line_index == 0 {
                        curr = self.get_state_by_name(c);
                    } else {
                        if self.values.contains(&c) {
                            val = c; 
                        }
                        if c == '0' {
                            dir = '0';
                            self.list[curr.unwrap()].end_state = true;
                        } else if self.has_state_by_name(c) {
                            dir = c;
                        }
                    }
                }
            }
            if dir != '`' && val != '`' {
                self.list[curr.unwrap()].directions.insert(dir.to_string(), val);
            }
        }
    }

    pub fn get_starting_state(&self) -> Option<&State> {
        for st in self.list.iter() {
            if st.start_state == true {
                return Some(st);
            }
        }
        None
    }
}

impl fmt::Display for StateList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for st in self.list.iter() {
            write!(f, "{}\n", st)?;
        }
        write!(f, " ")
    }
}
