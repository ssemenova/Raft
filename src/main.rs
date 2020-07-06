use std::time::{Duration};
use std::sync::mpsc;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use event_listener::Event;
use std::sync::Mutex;

mod timer;
mod state;
mod network;
mod rpc;

pub struct Flags {
    pub start_election: Arc<AtomicBool>,
    pub reset_timer: Arc<AtomicBool>,
    pub become_leader: Arc<AtomicBool>,
    pub demote_from_leader: Arc<AtomicBool>
}

pub struct Events {
    pub start_election: Arc<Event>,
    pub reset_timer: Arc<Event>,
    pub become_leader: Arc<Event>,
    pub demote_from_leader: Arc<Event>
}

fn main() {
    // Set up node state
    let node_state = state::State::new();

    // Flags and events for inter-thread communication    
    // [start election] from timer to main thread
    // [reset timer] from main thread to timer
    // [send heartbeat] from start election thread
    // [stop heartbeats] from network thread to heartbeats thread
    let flags = Flags {
        start_election: Arc::new(AtomicBool::new(false)),
        reset_timer: Arc::new(AtomicBool::new(false)),
        become_leader: Arc::new(AtomicBool::new(false)),
        demote_from_leader: Arc::new(AtomicBool::new(false)),
    };
    let events = Events {
        start_election: Arc::new(Event::new()),
        reset_timer: Arc::new(Event::new()),
        become_leader: Arc::new(Event::new()),
        demote_from_leader: Arc::new(Event::new()),
    };

    // Start threads
    let _election_timer = timer::Timer::start(
        flags.start_election.clone(),
        events.start_election.clone(),
        flags.reset_timer.clone(),
    );
    receive_on_network();

    // Listen for events from other threads
    loop {
        if flags.start_election.load(Ordering::SeqCst) {
            println!("start election flag received");
            flags.start_election.store(false, Ordering::SeqCst);
            start_election(
                flags.demote_from_leader.clone(),
                flags.become_leader.clone(),
                events.become_leader.clone()
            );
        }
        if flags.become_leader.load(Ordering::SeqCst) {
            // If node type upgrades to a leader, start sending heartbeats
            println!("send heartbeats flag received");
            flags.become_leader.store(false, Ordering::SeqCst);
            start_sending_heartbeats_thread(flags.become_leader.clone());
        }
    }
}

fn receive_on_network() {
    // "service client requests/respond to RPCs" thread

    // TODO: If a node receives any message before the 
    // election timer runs out, reset the timer 
    // reset_timer_flag.store(true, Ordering::SeqCst);
    // reset_timer_event.notify(usize::MAX);
}

fn start_sending_heartbeats_thread(demote_from_leader_flag: Arc<AtomicBool>) {
    loop {
        // TODO: Send a heartbeat to every node every few seconds
    
        // If node type demotes down from a leader, stop sending heartbeats
        if demote_from_leader_flag.load(Ordering::SeqCst) {
            println!("stop heartbeats flag received");
            demote_from_leader_flag.store(false, Ordering::SeqCst);
            break;
        }
    }
}

fn start_election(
    demote_from_leader_flag: Arc<AtomicBool>, 
    become_leader_flag: Arc<AtomicBool>,
    become_leader_event: Arc<Event>
) {
    // TODO: A new thread should start to begin the leader election process.

    // Can be interrupted if the node receives an AppendEntries RPC and it’s from a valid leader.
    // There can only be one leader, so stop the leader election process.
    if demote_from_leader_flag.load(Ordering::SeqCst) {
        println!("stop heartbeats flag received");
        demote_from_leader_flag.store(false, Ordering::SeqCst);
        return;
    }

    // TODO: See leader election section of paper for implementation details

    // TODO: After successfully becoming a leader, update the node state to reflect its leadership. 
    become_leader_flag.store(true, Ordering::SeqCst);
    become_leader_event.notify(usize::MAX);

    // TODO: kick off a heartbeat sender thread and a service client requests thread. 
}