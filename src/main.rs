#[macro_use]
extern crate log;
extern crate env_logger;

use std::collections::HashSet;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;

type URI = String;

struct WorkQueue<T> {
    frontier: Vec<T>,
    working: u8,
}

impl<T> WorkQueue<T> {
    fn start_work(&mut self) {
        self.working += 1
    }

    fn stop_work(&mut self) {
        self.working -= 1
    }

    fn is_finished(&self) -> bool {
        self.working == 0
    }
}

fn get_links(uri: &URI) -> HashSet<URI> {
    thread::sleep_ms(3000);
    let mut s = HashSet::new();
    s.insert(uri.clone());
    s.insert("sup".to_string());
    s.insert("bar".to_string());
    s
}

fn crawl(root_uri: URI) -> HashSet<URI> {
    let mut visited = HashSet::new();

    {
        let shared_visited = Arc::new(RwLock::new(&mut visited));

        let mut frontier = Vec::new();
        frontier.push(root_uri);

        let work_queue = WorkQueue { frontier: frontier, working: 0 };

        let shared = Arc::new((Mutex::new(work_queue), Condvar::new()));

        let _gs: Vec<_> = (0..2).map(|i| {
            let visited_lock = shared_visited.clone();

            let cloned = shared.clone();

            thread::scoped(move || {
                let (ref work_lock, ref condvar) = *cloned;

                'work: loop {
                    let uri: URI;
                    {
                        debug!("Thread {}: locking `work_lock`", i);
                        let mut work_guard = work_lock.lock().unwrap();
                        debug!("Thread {}: locked `work_lock`", i);

                        'pop: loop {
                            match work_guard.frontier.pop() {
                                Some(next) => {
                                    debug!("Thread {}: popped value {:?}", i, next);
                                    work_guard.start_work();
                                    uri = next;
                                    break 'pop
                                },
                                None => {
                                    debug!("Thread {}: other working threads: {}", i, work_guard.working);
                                    if work_guard.is_finished() {
                                        break 'work
                                    } else {
                                        debug!("Thread {}: going to sleep!", i);
                                        work_guard = condvar.wait(work_guard).unwrap();
                                        debug!("Thread {}: waking up!", i);
                                    }
                                }
                            }
                        }
                    }

                    let links = get_links(&uri);

                    visited_lock.write().unwrap().insert(uri);

                    let mut work_guard = work_lock.lock().unwrap();
                    if !links.is_empty() {
                        let visited_read = visited_lock.read().unwrap();

                        for link in links {
                            if !visited_read.contains(&link) { work_guard.frontier.push(link) }
                        }
                    }
                    work_guard.stop_work();
                    condvar.notify_all();
                }
            })
        }).collect();
    }

    visited
}

fn main() {
    env_logger::init().unwrap();

    println!("{:?}", crawl("bro".to_string()));
}
