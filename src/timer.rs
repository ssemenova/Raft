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

          let mut state_lock = timer.node_state.lock().unwrap();

          if elapsed_time >= timer.max_duration {
            timer.start = Instant::now();
            state_lock.start_election = true;
            state_lock.node_type = 1;
          }

          if state_lock.reset_timer {
            timer.start = Instant::now(); 
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