mod task;
mod dispatcher;
mod worker;
mod monitor;
mod metrics;

use dispatcher::{run_fifo_dispatcher, run_optimized_dispatcher};
use metrics::Metrics;
use monitor::start_monitor;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use task::generate_exact_tasks;
use worker::start_worker_pool;

fn run_simulation(name: &str, io_count: usize, cpu_count: usize, optimized: bool) {
    println!("\n========================================");
    println!("Starting simulation: {}", name);
    println!("IO tasks: {} | CPU tasks: {}", io_count, cpu_count);
    println!("========================================");

    let simulation_start = Instant::now();

    let total_tasks = io_count + cpu_count;
    let tasks = generate_exact_tasks(io_count, cpu_count);

    let current_cpu = Arc::new(Mutex::new(0u32));
    let metrics = Arc::new(Mutex::new(Metrics::new()));

    let monitor_handle = start_monitor(
        Arc::clone(&current_cpu),
        Arc::clone(&metrics),
        total_tasks,
    );

    let (task_sender, task_receiver) = mpsc::channel();
    let (worker_sender, worker_receiver) = mpsc::channel();

    let shared_worker_receiver = Arc::new(Mutex::new(worker_receiver));

    let worker_handles = start_worker_pool(
        8,
        shared_worker_receiver,
        Arc::clone(&current_cpu),
        Arc::clone(&metrics),
    );

    let dispatcher_cpu = Arc::clone(&current_cpu);

    let dispatcher_handle = if optimized {
    thread::spawn(move || {
        run_optimized_dispatcher(task_receiver, worker_sender, dispatcher_cpu);
    })
} else {
    thread::spawn(move || {
        run_fifo_dispatcher(task_receiver, worker_sender, dispatcher_cpu);
    })
};

    for mut task in tasks {
        task.arrival_time = Instant::now();
        task_sender.send(task).unwrap();
        thread::sleep(Duration::from_millis(20));
    }

    drop(task_sender);

    dispatcher_handle.join().unwrap();

    for handle in worker_handles {
        handle.join().unwrap();
    }

    monitor_handle.join().unwrap();

    let total_runtime_ms = simulation_start.elapsed().as_millis();

    let final_metrics = metrics.lock().unwrap();
    final_metrics.print_summary(total_runtime_ms);

    println!("Finished simulation: {}", name);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Choose a simulation:");
        println!("cargo run -- 700");
        println!("cargo run -- 800");
        println!("cargo run -- opt700");
        println!("cargo run -- opt800");
        println!("cargo run -- both");
        return;
    }

    match args[1].as_str() {
    "700" => run_simulation("FIFO 700 IO / 300 CPU", 700, 300, false),
    "800" => run_simulation("FIFO 800 IO / 200 CPU", 800, 200, false),

    "opt700" => run_simulation("OPT 700 IO / 300 CPU", 700, 300, true),
    "opt800" => run_simulation("OPT 800 IO / 200 CPU", 800, 200, true),

    "both" => {
        run_simulation("FIFO 700 IO / 300 CPU", 700, 300, false);
        run_simulation("OPT 700 IO / 300 CPU", 700, 300, true);
    }

    _ => {
        println!("Use:");
        println!("cargo run -- 700");
        println!("cargo run -- 800");
        println!("cargo run -- opt700");
        println!("cargo run -- opt800");
    }
    }
}