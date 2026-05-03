mod task;
mod dispatcher;
mod worker;
mod monitor;
mod metrics;

use dispatcher::run_fifo_dispatcher;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use task::generate_tasks;
use worker::start_worker_pool;

fn main() {
    println!("Task Dispatcher Simulation Starting...");

    let tasks = generate_tasks(100, 0.7);

    let current_cpu = Arc::new(Mutex::new(0u32));

    let (task_sender, task_receiver) = mpsc::channel();
    let (worker_sender, worker_receiver) = mpsc::channel();

    let shared_worker_receiver = Arc::new(Mutex::new(worker_receiver));

    let worker_handles = start_worker_pool(
        8,
        shared_worker_receiver,
        Arc::clone(&current_cpu),
    );

    let dispatcher_cpu = Arc::clone(&current_cpu);

    let dispatcher_handle = thread::spawn(move || {
        run_fifo_dispatcher(task_receiver, worker_sender, dispatcher_cpu);
    });

    for task in tasks {
        task_sender.send(task).unwrap();
        thread::sleep(Duration::from_millis(20));
    }

    drop(task_sender);

    dispatcher_handle.join().unwrap();

    for handle in worker_handles {
        handle.join().unwrap();
    }

    println!("Simulation complete.");
}