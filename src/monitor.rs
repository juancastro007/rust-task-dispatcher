use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn start_monitor(
    current_cpu: Arc<Mutex<u32>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let start_time = Instant::now();

        loop {
            let cpu = {
                let cpu_lock = current_cpu.lock().unwrap();
                *cpu_lock
            };

            let elapsed = start_time.elapsed().as_millis();

            println!(
                "[MONITOR] Time: {} ms | CPU Usage: {}%",
                elapsed, cpu
            );

            // Stop condition (simple version)
            if cpu == 0 && elapsed > 3000 {
                println!("[MONITOR] Shutting down");
                break;
            }

            thread::sleep(Duration::from_millis(10));
        }
    })
}