use crate::metrics::Metrics;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn start_monitor(
    current_cpu: Arc<Mutex<u32>>,
    metrics: Arc<Mutex<Metrics>>,
    total_tasks: usize,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let start_time = Instant::now();

        loop {
            let cpu = {
                let cpu_lock = current_cpu.lock().unwrap();
                *cpu_lock
            };

            let completed = {
                let metrics_lock = metrics.lock().unwrap();
                metrics_lock.completed_tasks
            };

            let elapsed = start_time.elapsed().as_millis();

            println!(
                "[MONITOR] Time: {} ms | CPU Usage: {}% | Completed: {}",
                elapsed, cpu, completed
            );

            // ✅ Proper stop condition
            if completed >= total_tasks && cpu == 0 {
                println!("[MONITOR] Shutting down");
                break;
            }

            thread::sleep(Duration::from_millis(10));
        }
    })
}