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
    println!("zz{}", state_list);

    let beg_state = state_list.get_starting_state().unwrap();
    let mut x: Vec<&states::State> = vec![beg_state];
    let y: Vec<&states::State> = vec![beg_state, &state_list.list[2]];
    let mut dfa_1 = dfa::DFAState::new_dfa_state(&x);
    let mut dfa_2 = dfa::DFAState::new_dfa_state(&y);


    let mut dfalist = dfa::DFAList::new();
    dfalist.append_dfa_state(dfa_1);
    dfalist.append_dfa_state(dfa_2);

    println!("{}", dfalist);
    dfalist.get_terminals(&state_list);
    println!("{:?}", dfalist.terminals);
    dfalist.check_connections(&state_list);
}
