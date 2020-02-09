mod models;
use models::states::{StateList};
use models::read_grammar::{read_states};
use models::traverse::{check_string};

fn main() {
    let mut state_list = StateList::new();
    read_states(&mut state_list, "gramm.txt".to_string());
    state_list.read_directions("gramm.txt".to_string());
    // println!("{}", state_list);

    println!("{}", check_string(state_list, "aababaabaaaa".to_string()));

}