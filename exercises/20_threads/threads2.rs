// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));

    let handles = (0..10).map(|_| {
        let status_shared = Arc::clone(&status);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            match status_shared.lock() {
                Ok(mut status) => {
                    status.jobs_completed += 1;
                }
                Err(_) => {
                    println!("thread failed to acquire lock");
                }
            }
        })
    });

    handles.for_each(|handle| match handle.join() {
        Ok(_) => match status.lock() {
            Ok(status) => {
                println!("jobs completed {}", status.jobs_completed);
            }
            Err(_) => {
                println!("thread failed to acquire lock");
            }
        },
        Err(_) => {
            println!("thread failed to join");
        }
    });
}
