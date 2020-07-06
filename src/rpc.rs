use std::vec::Vec;

pub fn append_entries(
term: i32, leader_ID: i32, prev_log_index: i32, prev_log_term: i32,
entries: Vec<i32>, leader_commit: i32) -> (i32, bool) {

  //dummy return value
  return (1, true);
}

pub fn install_snapshots(
  term: i32, leader_ID: i32, last_included_index: i32,
  last_included_term: i32, offset: i32, data: Vec<i32>,
  done: bool
) -> i32 {

  //dummy return value
  return 1;
}

pub fn request_vote(
  term: i32, candidate_id: i32, last_log_index: i32, last_log_term: i32
) -> (i32, bool) {

  // dummy return value
  return (1, true);
}