use std::vec::Vec;

pub struct State {
  // Persistent State

  // latest term server has seen 
  // (initialized to 0 on first boot, increases monotonically)
  pub current_term: i32,
  // candidate Id that received vote in current term
  // (or null if none)
  pub voted_for: Option<i32>,
  // log entries. Each entry contains command for state machine
  // and term when entry was received by leader
  // (first index is 1)
  pub log: Vec<(String, i32)>,
  
  // Volatile state on all servers
  
  // index of highest log entry known to be committed
  // (initialized to 0, increases monotonically)
  pub commit_index: i32,
  // index of highest log entry applied to state machine
  // (initialized to 0, increases monotonically)
  last_applied: i32,
  
  // Volatile state on leaders
  // (re-initialized after election)

  // for each server, index of the next log entry to send
  // to that server
  // (initialized to leader last log index + 1)
  next_index: Vec<i32>,
  // for each server, index of highest log entry 
  // known to be replicated on the server
  // (initialized to 0, increases monotonically)
  match_index: Vec<i32>,

  // Specific to our implementation
  pub node_type: i32,

  // Flags for inter-thread communication
  pub reset_timer: bool,
  pub start_election: bool,
  pub propagate_message_in_heartbeat: bool,

  // Vote count, used between threads
  pub vote_count: i32,

  // List of IP addresses and ports in the cluster
  pub cluster: Vec<String>,
  pub my_networking_info: String,

  pub leader: String
}

impl State {
  pub fn new(cluster: Vec<String>, my_networking_info: String) -> State {
    let mut log = Vec::new();

    let mut next_index = Vec::new();
    next_index.push(1);

    let mut match_index = Vec::new();
    match_index.push(0);


    let mut state = State {
      current_term: 0,
      voted_for: None,
      log: log,
      commit_index: 0,
      last_applied: 0,
      next_index: next_index,
      match_index: match_index,
      node_type: 1, // Follower, Candidate, Leader
      reset_timer: false,
      start_election: false,
      propagate_message_in_heartbeat: false,
      cluster: cluster,
      my_networking_info: my_networking_info,
      leader: "".to_string(), // TODO: how to decide a leader at the very beginning?,
      vote_count: 0
    };

    return state;
  }

}