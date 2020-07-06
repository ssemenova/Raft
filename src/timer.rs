use rand::Rng;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use event_listener::Event;
use std::sync::Arc;

pub struct Timer {
  pub start: Instant,
  pub max_duration: u128,
  pub start_election_flag: Arc<AtomicBool>,
  pub start_election_event: Arc<Event>,
  pub reset_timer_flag: Arc<AtomicBool>,
}

impl Timer {
  pub fn start(
    start_election_flag: Arc<AtomicBool>,
    start_election_event: Arc<Event>,
    reset_timer_flag: Arc<AtomicBool>
  ) {
    thread::spawn({
      let mut timer = Timer {
        start: Instant::now(),
        max_duration: Self::get_random_timer_duration(),
        start_election_flag: start_election_flag.clone(),
        start_election_event: start_election_event.clone(),
        reset_timer_flag: reset_timer_flag.clone()
      };

      move || {
        loop {
          let elapsed_time = timer.start.elapsed().as_millis();

          if elapsed_time >= timer.max_duration {
            timer.start_election();
          }

          if timer.reset_timer_flag_received() {
            timer.reset_timer();
          }

        }
      }
    });
  }

  fn reset_timer_flag_received(&self) -> bool {
    return self.reset_timer_flag.load(Ordering::SeqCst);
  }

  fn reset_timer(&mut self) {
    self.reset_timer_flag.store(false, Ordering::SeqCst);
    self.start = Instant::now();
  }

  fn start_election(&mut self) {
    self.start = Instant::now();
    self.start_election_flag.store(true, Ordering::SeqCst);
    self.start_election_event.notify(usize::MAX);
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