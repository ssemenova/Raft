use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use rand::Rng;

use crate::state;

pub struct Timer {
  start: Instant,
  max_duration: u128,
  node_state: Arc<Mutex<state::State>>
}

impl Timer {
  pub fn start(
    node_state: &Arc<Mutex<state::State>>
  ) {
    thread::spawn({
      let mut timer = Timer {
        start: Instant::now(),
        max_duration: Self::get_random_timer_duration(),
        node_state: Arc::clone(&node_state)
      };

      move || {
        loop {
          let elapsed_time = timer.start.elapsed().as_millis();

          // If election timeout elapses without receiving AppendEntries RPC
          // or granting vote to candidate, convert to candidate
          if elapsed_time >= timer.max_duration {
            timer.start = Instant::now();
            let mut state_lock = timer.node_state.lock().unwrap();
            state_lock.start_election = true;
            state_lock.node_type = 2;
          }

          let reset_timer;
          {
            let state_lock = timer.node_state.lock().unwrap();
            reset_timer = state_lock.reset_timer;
          }

          // Flag is set if a message is sent from another leader
          if reset_timer {
            timer.start = Instant::now(); 
            let mut state_lock = timer.node_state.lock().unwrap();
            state_lock.reset_timer = false;
          }
        }
      }
    });
  }

  fn get_random_timer_duration() -> u128 {
    // Paper instantiates each node with a random election timer
    // between 150 and 300 ms
    let mut rng = rand::thread_rng();

    // made this 10x longer for now, easier to see what's going on this way
    // let ms = rng.gen_range(150, 300);
    let ms = rng.gen_range(1500, 3000);

    return Duration::from_millis(ms).as_millis();
  }
}