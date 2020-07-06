use std::vec::Vec;

pub struct State {
  // Persistent State

  // latest term server has seen 
  // (initialized to 0 on first boot, increases monotonically)
  current_term: i32,
  // candidate Id that received vote in current term
  // (or null if none)
  voted_for: Option<i32>,
  // log entries. Each entry contains command for state machine
  // and term when entry was received by leader
  // (first index is 1)
  log: Vec<(String, i32)>,
  
  // Volatile state on all servers
  
  // index of highest log entry known to be committed
  // (initialized to 0, increases monotonically)
  commit_index: i32,
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
  node_type: NodeType,
}

enum NodeType {
  Leader,
  Coordinator,
  Follower  
}

impl State {
  pub fn new() -> State {
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
      node_type: NodeType::Coordinator,
    };

    return state;
  }
}