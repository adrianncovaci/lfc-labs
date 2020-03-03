use crate::models::states;
use crate::models::dfa;

pub fn convert_nfa_to_dfa(state_list: &states::StateList) {
    let mut dfa_list = dfa::DFAList::new();
    // let mut vector: Vec<&states::State> = vec![state_list.get_starting_state().unwrap()];
    // let mut dfa_state = dfa::DFAState::new_dfa_state(&vector);

    // dfa_list.append_dfa_state(dfa_state);
    // for mut dfa_sts in &dfa_list.states {
    //     for i in &state_list.values {
    //         let vecz = check_connections(&mut dfa_sts, *i, &state_list);
    //         let mut dfa_st = dfa::DFAState::new_dfa_state(&vecz);
    //         dfa_list.append_dfa_state(*dfa_sts);
    //     }
    // }
   
}

pub fn check_connections<'a>(state: &'a mut dfa::DFAState, state_list: &'a states::StateList) -> Vec<&'a states::State> {
    let mut vec: Vec<&'a states::State> = Vec::new();
//         for i in &state_list.values {
//             let mut string = String::new();
//             let ch = i;
//             for st in state.name.iter() {
//                 for (key, val) in &st.directions {
//                     if *val == *ch {
//                         string.push_str(&key);
//                         let str_val: Vec<char> = key.chars().collect();
//                         let node: &states::State = &state_list.list[state_list.get_state_by_name(str_val[0]).unwrap()];
//                         if !vec.contains(&node) {
//                             vec.push(&node);
//                         }
//                     }
//                 }
//             }
//             if string != "".to_string() {
//                 state.link.insert(string, *ch);
//             }
//         }
//     println!("q{} {:?} {}", state, state.link, vec.len());
    vec
}
