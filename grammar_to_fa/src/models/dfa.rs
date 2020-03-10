use crate::models::states::{State, StateList};
use std::fmt;
use std::collections::HashMap;
use std::sync::Arc;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};



#[derive(PartialEq, Eq, Clone)]
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

    pub fn new_dfa_state(states: Vec<&'a State>) -> DFAState<'a> {
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

    fn append_states(&mut self,states: &Vec<&'a State>) {
        for x in states {
            if !self.name.contains(&x) {
                self.name.push(&x);
            }
        }
    }

    pub fn setup_link(&mut self, chrs: &Vec<char>, state_list: &'a StateList) -> Vec<DFAState<'a>> {
        let mut dfa_vec = vec![];
        let mut vec_names = vec![];
        let mut string = String::new();
        for ch in chrs {

        let mut string = String::new();
        let mut vec = vec![];
        for state in self.name.iter() {
            let mut name_string = String::new();
            for (key, val) in &state.directions {
                if *val == *ch {
                    if !string.contains(&key.as_str()) {
                        string.push_str(&key);
                    }
                    let str_val: Vec<char> = key.chars().collect();
                    let node: &State = &state_list.list[state_list.get_state_by_name(str_val[0]).unwrap()];
                    let drow: String = string.chars().rev().collect::<String>();
               

                    if !vec.contains(&node) && !vec_names.contains(&string) && !vec_names.contains(&drow) {
                        vec.push(&node);
                        vec_names.push(string.clone());
                    }

                }
            }

        }
            if string != "".to_string() {
                    self.link.insert(string, *ch);
            }
            if vec.len() > 0 {
        let x = DFAState {
            end_state: DFAState::contains_end_state(&vec),
            name: vec.to_vec(),
            link: HashMap::new(),
        };
        if !dfa_vec.contains(&x) {
            dfa_vec.push(x);
        }
            }
        }
        dfa_vec
    }
}


impl<'a> fmt::Display for DFAState<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for st in self.name.iter() {
            write!(f, "{}", st)?;
        }
        write!(f, "=> {:?} ", self.link)
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

    pub fn check_if_contains(&self, list: &DFAState<'a>) -> bool {
        for x in &list.name {
            for y in &self.states {
                for z in &y.name {
                if *x != *z {
                    return false;
                }
                }
            }
        }
        return true;
    }

    pub fn check_connections(&mut self, state_list: &'a StateList) {
        let mut dfa_vec = vec![];
            for stmnt in &mut self.states {
                if stmnt.link.is_empty() {
                    let new_dfa_state = stmnt.setup_link(&self.terminals, state_list);
                    for x in &new_dfa_state {
                    }
                        dfa_vec.push(new_dfa_state);
                }
            }

        let mut checker: bool = true;

        for statements in dfa_vec {
            for x in statements {
                if !self.check_if_contains(&x) {
                    self.append_dfa_state(x);
                    checker = false;
                }
        }

        for x in &self.states {
        }

        if checker {
            self.check_connections(state_list);
        }
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
