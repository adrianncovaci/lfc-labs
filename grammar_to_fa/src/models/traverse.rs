use super::states::{StateList};

pub fn check_string(state_list: &StateList, content: String) -> bool {
    let mut curr = state_list.get_starting_state().unwrap();
    for chr in content.chars() {
        if curr.directions.contains_key(&chr.to_string()) {
            let dir = curr.directions.get(&chr.to_string()).unwrap();
            let ind = state_list.get_state_by_name(*dir).unwrap();
            curr = &state_list.list[ind];
        } else {
            return false;
        }
    }
    return true;
}
