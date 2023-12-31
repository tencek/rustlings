// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

use std::thread;
use std::time::{Duration, Instant};

type Durations = Vec<u128>;

fn main() {
    let handles = (0..10).map(|i| {
        thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            // start.elapsed().as_millis()
            panic!("thread {i} failed");
        })
    });

    // TODO: a struct is returned from thread::spawn, can you use it?
    let result: Result<Durations, _> = handles.map(|handle| handle.join()).collect();

    match result {
        Ok(durations) => {
            println!();
            for (i, duration) in durations.into_iter().enumerate() {
                println!("thread {} took {}ms", i, duration);
            }
        }
        Err(threadError) => eprintln!(
            "Oh no! All the spawned threads did not finish! {:?}",
            threadError
        ),
    }
}
