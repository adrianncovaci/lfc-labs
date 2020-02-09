use std::fs::File;
use std::io::{BufReader, BufRead};
use super::*;

pub fn read_states(state_list:&mut states::StateList, filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        for (line_index, c) in line.chars().enumerate() {
            if index == 0 && c != ' ' {
                state_list.append_state(states::State::new(c));
            }
            if index == 0 && c != ' ' && state_list.list.len() == 1 {
                state_list.list[line_index].start_state = true;
            }
            if index == 1 {
                state_list.read_values(&line);
            }
        }
    }
}