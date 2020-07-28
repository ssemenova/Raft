use std::cmp;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

use crate::state;

pub fn append_entries(
node_state: &Arc<Mutex<state::State>>,
term: i32, leader_ID: i32, prev_log_index: usize, prev_log_term: i32,
entries: Vec<i32>, leader_commit: i32) -> (i32, bool) {
  let current_term; let log_term;
  {
    let state_lock = node_state.lock().unwrap();
    current_term = state_lock.current_term;
    log_term = state_lock.log[prev_log_index].1;
  }

  if term < current_term {
    return (current_term, false);
  }
    
  if log_term != prev_log_term {
    return (current_term, false);
  }

  for entry in entries {
    // TODO: 
    // if an existing entry conflicts with a new one
    // (where a conflict is that two entries have the same index but different terms)
    // delete the existing entry and all that follow it
    // need clarification for what "all that follow it" means...
    // should all that follow that entry in "entries" be deleted?
    // or in state_lock.log ?
  }

  // TODO: append any new entries not already in the log

  let commit_index;
  {
    let state_lock = node_state.lock().unwrap();
    commit_index = state_lock.commit_index;
  }

  // TODO: get index of last new entry
  let index_of_last_new_entry;

  if leader_commit > commit_index {
    let mut state_lock = node_state.lock().unwrap();
    state_lock.commit_index = cmp::min(
      leader_commit, index_of_last_new_entry
    );

  }

  return (current_term, true);
}

pub fn install_snapshots(
  term: i32, leader_ID: i32, last_included_index: i32,
  last_included_term: i32, offset: i32, data: Vec<i32>,
  done: bool
) -> i32 {

  // TODO: finish this function
  return 1;
}

pub fn request_vote(
  node_state: &Arc<Mutex<state::State>>,
  term: i32, candidate_id: i32, last_log_index: i32, last_log_term: i32
) -> (i32, bool) {
  let current_term; let voted_for;
  {
    let state_lock = node_state.lock().unwrap();
    current_term = state_lock.current_term;
    voted_for = state_lock.voted_for;
  }

  if term < current_term {
    return (current_term, false);
  }

  let mut null_or_candidate_id = false;
  match voted_for {
    Some(val) => {
      if val == candidate_id {
        null_or_candidate_id = true;
      }
    },
    None => null_or_candidate_id = true
  }

  // if null_or_candidate_id and 
  // candidate's log is at least as up to date as receiver's log
  // grant vote
  // section 5.2, 5.4
  // TODO: how to decide if candidate's log is "at least up to date" as receiver's log?
  if null_or_candidate_id {
    return (current_term, true);
  } else {
    return (current_term, false);
  }
}