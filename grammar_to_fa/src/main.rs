mod models;
use models::states::{StateList};
use models::states;
use models::read_grammar::{read_states};
use models::traverse::{check_string};
use models::dfa;
use models::nfa_to_dfa::{ check_connections};

fn main() {
    let mut state_list = StateList::new();
    read_states(&mut state_list, "gramm.txt".to_string());
    state_list.read_directions("gramm.txt".to_string());
    println!("{}", check_string(&state_list, "aababaabaaaa".to_string()));
    let beg_state = state_list.get_starting_state().unwrap();
    let mut x: Vec<&states::State> = vec![beg_state];
    let mut dfa_1 = dfa::DFAState::new_dfa_state(x);

    let mut dfalist = dfa::DFAList::new();
    dfalist.append_dfa_state(dfa_1);
    dfalist.get_terminals(&state_list);
    dfalist.check_connections(&state_list);
    dfalist.check_connections(&state_list);
    dfalist.check_connections(&state_list);

    println!("{}", dfalist);
}
