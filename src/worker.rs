use crate::task::{Task, TaskKind};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_worker_pool(
    worker_count: usize,
    receiver: Arc<Mutex<mpsc::Receiver<Task>>>,
    current_cpu: Arc<Mutex<u32>>,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::new();

    for worker_id in 0..worker_count {
        let receiver_clone = Arc::clone(&receiver);
        let cpu_clone = Arc::clone(&current_cpu);

        let handle = thread::spawn(move || loop {
            let task_result = {
                let locked_receiver = receiver_clone.lock().unwrap();
                locked_receiver.recv()
            };

            match task_result {
                Ok(task) => {
                    match task.kind {
                        TaskKind::IO => {
                            println!("Worker {} running IO task {}", worker_id, task.id);
                        }
                        TaskKind::CPU => {
                            println!("Worker {} running CPU task {}", worker_id, task.id);
                        }
                    }

                    thread::sleep(Duration::from_millis(task.duration_ms));

                    {
                        let mut cpu = cpu_clone.lock().unwrap();
                        *cpu -= task.cpu_cost;
                        println!(
                            "Worker {} completed task {} | CPU now {}%",
                            worker_id, task.id, *cpu
                        );
                    }
                }

                Err(_) => {
                    println!("Worker {} shutting down", worker_id);
                    break;
                }
            }
        });

        handles.push(handle);
    }

    handles
}