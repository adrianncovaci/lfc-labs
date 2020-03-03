use crate::models::states::{State, StateList};
use std::fmt;
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
pub struct DFAState<'a> {
    pub name: Vec<&'a State>,
    pub link: HashMap<String, char>,
    pub end_state: bool,
}

impl<'a> DFAState<'a> {
    pub fn new() -> DFAState<'a> {
        DFAState {
            name: Vec::new(),
            link: HashMap::new(),
            end_state: false,
        }
    }

    pub fn new_dfa_state(states: &'a Vec<&State>) -> DFAState<'a> {
        let val = DFAState::contains_end_state(&states);

        DFAState {
            name: states.to_vec(),
            end_state: val,
            link: HashMap::new(),
        }
    }

    fn contains_end_state(states: &Vec<&State>) -> bool {

        for x in states {
            if x.end_state == true {
                return true;
            }
        }

        false
    }
}

impl<'a> fmt::Display for DFAState<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for st in self.name.iter() {
            write!(f, "{}", st)?;
        }
        write!(f, " ")
    }
}

pub struct DFAList<'a> {
    pub states: Vec<DFAState<'a>>,
    pub terminals: Vec<char>,
}

impl<'a> DFAList<'a> {
    pub fn new() -> DFAList<'a> {
        DFAList {
            states: Vec::new(),
            terminals: Vec::new(),
        }
    }

    pub fn append_dfa_state(&mut self, val: DFAState<'a>) {
        if !self.states.contains(&val) {
            self.states.push(val);
        }
    }

    pub fn get_starting_state(&self) -> Option<&DFAState> {
        for x in &self.states {
            if x.name.len() == 1 {
                for inner_state in &x.name {
                    if inner_state.start_state {
                        return Some(&x);
                    }
                }
            }
        }
        return None;
    }

    pub fn get_terminals(&mut self, list: &StateList) {
        let mut v = list.values.clone();
        v.sort();
        v.dedup();
        self.terminals = v.clone();
    }

    pub fn check_connections(&mut self, state_list: &'a StateList) {
        for mut stmnt in self.states {
        let mut vec: Vec<&'a State> = Vec::new();
            for i in &state_list.values {
                let mut string = String::new();
                let ch = i;
                for st in stmnt.name.iter() {
                    for (key, val) in &st.directions {
                        if *val == *ch {
                            string.push_str(&key);
                            let str_val: Vec<char> = key.chars().collect();
                            let node: &State = &state_list.list[state_list.get_state_by_name(str_val[0]).unwrap()];
                            if !vec.contains(&node) {
                                vec.push(&node);
                            }
                        }
                    }
                }
                if string != "".to_string() {
                    stmnt.link.insert(string, *ch);
                }
            }
        println!("q{} {:?} {}", stmnt, stmnt.link, vec.len());
            self.states.push(DFAState::new_dfa_state(&vec));
        }
    }
}

impl<'a> fmt::Display for DFAList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for st in self.states.iter() {
            write!(f, "{}\n", st)?;
        }
        write!(f, " ")
    }
}
