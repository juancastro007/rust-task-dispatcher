use crate::task::Task;
use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run_fifo_dispatcher(
    incoming_receiver: mpsc::Receiver<Task>,
    worker_sender: mpsc::Sender<Task>,
    current_cpu: Arc<Mutex<u32>>,
) {
    let mut ready_queue: VecDeque<Task> = VecDeque::new();
    let mut incoming_closed = false;

    loop {
        while let Ok(task) = incoming_receiver.try_recv() {
            println!("Dispatcher received task {}", task.id);
            ready_queue.push_back(task);
        }

        if !incoming_closed {
            match incoming_receiver.try_recv() {
                Ok(task) => {
                    println!("Dispatcher received task {}", task.id);
                    ready_queue.push_back(task);
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    incoming_closed = true;
                }
            }
        }

        if let Some(task) = ready_queue.front() {
            let mut cpu = current_cpu.lock().unwrap();

            if *cpu + task.cpu_cost <= 100 {
                let task = ready_queue.pop_front().unwrap();
                *cpu += task.cpu_cost;

                println!(
                    "Dispatcher sending task {} | CPU now {}%",
                    task.id, *cpu
                );

                worker_sender.send(task).unwrap();
            }
        }

        if incoming_closed && ready_queue.is_empty() {
            println!("Dispatcher shutting down");
            break;
        }

        thread::sleep(Duration::from_millis(1));
    }
}