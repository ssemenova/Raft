use std::{env, thread, time};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

mod timer;
mod state;
mod rpc;
mod utils;

const HEARTBEAT_MSG: &str = "Heartbeat\n";
const APPEND_ENTRIES_MSG: &str = "Append Entries\n";
const REQUEST_VOTES_MSG: &str = "Request Votes\n";
const INSTALL_SNAPSHOTS_MSG: &str = "Install Snapshot\n";
const VOTE_MSG: &str = "Vote\n";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: ");
        println!(
            "cargo run ['IP address:my port number' for this server]
            ['IP address:port number' for any other server in the cluster]"
        );
        println!(
            "Example:
            'cargo run 127.0.0.1:1000 127.0.0.1:2000 127.0.0.1:3000"
        );
        return;
    }

    // TODO: is this the best way to handle all nodes joining the cluster in the beginning?
    // Doesn't quite feel right.
    let mut cluster = Vec::new();
    for n in 2..args.len() {
        cluster.push(args[n].to_string());
    }
    let my_networking_info = args[1].to_string();

    // Set up node state
    let node_state = Arc::new(Mutex::new(state::State::new(cluster, my_networking_info)));
    let mut prev_node_type = 1;

    // Start threads
    let _election_timer = timer::Timer::start(&node_state);
    receive_on_network_thread(&node_state);

    // Listen for events from other threads
    loop {
        let start_election; let current_node_type;
        {
            let state_lock = node_state.lock().unwrap();
            start_election = state_lock.start_election;
            current_node_type = state_lock.node_type;
        }

        if start_election {
            println!("start election flag received");
            let mut state_lock = node_state.lock().unwrap();
            state_lock.start_election = false;
            start_election_thread(&node_state);
        }
        if utils::node_promoted_to_leader(current_node_type, prev_node_type) {
            println!("node upgraded to leader; starting sending heartbeats");
            send_heartbeats_thread(&node_state);
        }

        prev_node_type = current_node_type;
    }
}

fn receive_on_network_thread(node_state: &Arc<Mutex<state::State>>) {
    let node_state = Arc::clone(node_state);

    thread::spawn(move || {
        let my_networking_info;
        {
            let state_lock = node_state.lock().unwrap();
            my_networking_info = state_lock.my_networking_info.to_string();
        }
        
        let listener = TcpListener::bind(my_networking_info).expect("could not start server");
        println!("Listening on network");

        for connection in listener.incoming() {
            match connection {
                Ok(mut stream) => {
                    let mut msg_type = String::new();
                    stream.read_to_string(&mut msg_type).expect("read failed");
                    println!("received message '{}'", msg_type);

                    let node_type;
                    {
                        let mut state_lock = node_state.lock().unwrap();
                        node_type = state_lock.node_type;
                        // If a node receives any message before the 
                        // election timer runs out, reset the timer 
                        state_lock.reset_timer = true;
                    }

                    // TODO: ingest more info from the caller
                    // info is dependent on the type of RPC that is sent

                    if node_type == 3 {
                        // append the command to the log as a new entry
                        // and inform the heartbeat thread to append the
                        // message in its AppendEntries RPC message
                        {
                            let mut state_lock = node_state.lock().unwrap();
                            // TODO: uncomment this when the correct info is ingested from the client
                            // let entry = (msg, state_lock.current_term);
                            // state_lock.log.push(entry);
                            // state_lock.propagate_message_in_heartbeat = true;
                        }

                        // TODO: finish this part

                        // After the entry is successfully replicated, the leader applies the entry to its state machine and returns the result to the client.
                        // Leader retries AppendEntries RPCs until all followers store all the entries.
                        // See “log replication” section for implementation details.
                    } else {
                        if msg_type == APPEND_ENTRIES_MSG {
                            {
                                let mut state_lock = node_state.lock().unwrap();
                                // if the appendentries RPC term is at least as large
                                // as the candidate's current term, then the candidate
                                // recognizes the legitimate leader and returns to
                                // follower state

                                // TODO: uncomment this when the correct info is ingested from the client
                                // if prev_log_term >= state_lock.current_term {
                                //     state_lock.node_type = 1;
                                // }
                            }
                            // TODO: uncomment this when the correct info is ingested from the client
                            // the correct info should be the arguments of this function
                            // rpc::append_entries(
                            //     &node_state, term, leader_ID, prev_log_index, prev_log_term, entries, leader_commit);
                        } else if msg_type == REQUEST_VOTES_MSG {
                            // TODO: uncomment this when the correct info is ingested from the client
                            // the correct info should be the arguments of this function
                            // rpc::request_vote(
                            //     &node_state, term, candidate_id, last_log_index, last_log_term
                            // );
                        } else if msg_type == INSTALL_SNAPSHOTS_MSG {
                            // TODO: uncomment this when the correct info is ingested from the client
                            // the correct info should be the arguments of this function
                            // rpc::install_snapshots(
                            //     term, leader_ID, last_included_index, last_included_term, offset, data, done
                            // );
                        } else if msg_type == VOTE_MSG {
                            let mut state_lock = node_state.lock().unwrap();
                            // TODO: finish responding to vote message
                            // this message is sent from a follower back to a candidate
                            // should increase the vote_count in global state
                            // and the election thread should listen for changes to the vote count
                        }
                    }
                }
                Err(e) => { println!("connection failed {}", e); }
            }
        }
    });
}

fn send_heartbeats_thread(node_state: &Arc<Mutex<state::State>>) {
    let node_state = Arc::clone(node_state);
    let sleep_time = time::Duration::from_millis(100);

    thread::spawn(move || {
        loop {
            let cluster;
            {
                let state_lock = node_state.lock().unwrap();
                cluster = state_lock.cluster.to_vec();
            }

            for client in cluster {
                let mut stream = TcpStream::connect(client).expect("connection failed");
                write!(stream, "{}", HEARTBEAT_MSG).expect("write failed");

                // If node type demotes down from a leader, stop sending heartbeats
                if !utils::node_still_leader(&node_state) {
                    return;
                }
            }
        
            thread::sleep(sleep_time);
        }
    });
}

fn start_election_thread(node_state: &Arc<Mutex<state::State>>) {
    let node_state = Arc::clone(node_state);

    thread::spawn(move || {
        let cluster;
        {
            let mut state_lock = node_state.lock().unwrap();
            state_lock.node_type = 2;
            state_lock.current_term += state_lock.current_term;
            state_lock.reset_timer = true;
            cluster = state_lock.cluster.to_vec();
        }

        let vote_count = 1;
        let majority_size = cluster.len() / 2;

        // TODO: execute these in parallel, if possible?
        for client in cluster {
            let mut stream = TcpStream::connect(client).expect("connection failed");
            write!(stream, "{}", REQUEST_VOTES_MSG).expect("write failed");

            // TODO: election response is sent from client back to this server
            // and ingested by the thread listening on a port. That thread modifies
            // the vote_count in the global state accordingly. 
            // This thread should ingest that info

            if vote_count > majority_size {
                println!("Election won, becoming leader.");
                let mut state_lock = node_state.lock().unwrap();
                state_lock.node_type = 3;

                return;
            }

            if !utils::node_still_candidate(&node_state) {
                return;
            }
        }

    });
}