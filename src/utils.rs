use std::sync::{Arc, Mutex};

use crate::state;

pub fn node_promoted_to_leader(
    current_node_type: i32, prev_node_type: i32
) -> bool {
    return current_node_type == 3 
            && prev_node_type == 1;
}

pub fn node_demoted_from_leader(
    current_node_type: i32, prev_node_type: i32
) -> bool {
    return (current_node_type == 1 || 
            current_node_type == 2)
            && prev_node_type == 3;
}

pub fn node_still_candidate(node_state: &Arc<Mutex<state::State>>) -> bool {
    let node_type;
    {
        let state_lock = node_state.lock().unwrap();
        node_type = state_lock.node_type;
    }

    if node_type == 1 {
        println!("Node received info about another leader; stopping leader election");
        return false;
    } else {
        return true;
    }
}

pub fn node_still_leader(node_state: &Arc<Mutex<state::State>>) -> bool {
    let node_type;
    {
        let state_lock = node_state.lock().unwrap();
        node_type = state_lock.node_type;
    }

    // If node type demotes down from a leader, stop sending heartbeats
    if node_type == 1 {
        println!("Node no longer leader; stop sending heartbeats");
        return false;
    }

    return true;
}
