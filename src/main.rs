use std::thread;
use std::sync::{Arc, Mutex};

mod timer;
mod state;
mod network;
mod rpc;
mod utils;

fn main() {
    // Set up node state
    let node_state = Arc::new(Mutex::new(state::State::new()));
    let mut prev_node_type = 1;

    // Start threads
    let _election_timer = timer::Timer::start(&node_state);
    receive_on_network(&node_state);

    // Listen for events from other threads
    loop {
        let mut state_lock = node_state.lock().unwrap();

        if state_lock.start_election {
            println!("start election flag received");
            state_lock.start_election = false;
            start_election(&node_state, prev_node_type);
        }
        if utils::node_promoted_to_leader(state_lock.node_type, prev_node_type) {
            // If node type upgrades to a leader, start sending heartbeats
            println!("node upgraded to leader; starting sending hearbeats");
            start_sending_heartbeats_thread(&node_state);
        }

        prev_node_type = state_lock.node_type;
    }
}

fn receive_on_network(node_state: &Arc<Mutex<state::State>>) {
    // "service client requests/respond to RPCs" thread
    let node_state = Arc::clone(node_state);

    thread::spawn(move || {
        let state_lock = node_state.lock().unwrap();

        loop {

            // TODO: If a node receives any message before the 
            // election timer runs out, reset the timer 
        }
    });

}

fn start_sending_heartbeats_thread(node_state: &Arc<Mutex<state::State>>) {
    let node_state = Arc::clone(node_state);

    thread::spawn(move || {
        loop {
            // TODO: Send a heartbeat to every node every few seconds
        
            // If node type demotes down from a leader, stop sending heartbeats
            let state_lock = node_state.lock().unwrap();
            if state_lock.node_type == 1 {
                println!("Node no longer leader; stop sending heartbeats");
                return;
            }
        }
    });
}

fn start_election(node_state: &Arc<Mutex<state::State>>, prev_node_type: i32) {
    let node_state = Arc::clone(node_state);

    thread::spawn(move || {
        let mut state_lock = node_state.lock().unwrap();
        state_lock.node_type = 2;

        // TODO: This condition should be checked every once in a while,
        // not just once in the election process. 
        // Can be interrupted if the node receives an AppendEntries RPC 
        // and it’s from a valid leader.
        // There can only be one leader, so stop the leader election process.
        let state_lock = node_state.lock().unwrap();
        if utils::node_demoted_from_leader(state_lock.node_type, prev_node_type) {
            println!("Node received info about another leader; stopping leader election");
            return;
        }

        // TODO: See leader election section of paper for implementation details

        // After successfully becoming a leader, 
        // tell main thread to update the node state to reflect its leadership.
        let mut state_lock = node_state.lock().unwrap();
        state_lock.node_type = 3;
    });
}
