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

pub fn run_optimized_dispatcher(
    incoming_receiver: mpsc::Receiver<Task>,
    worker_sender: mpsc::Sender<Task>,
    current_cpu: Arc<Mutex<u32>>,
) {
    let mut io_queue: VecDeque<Task> = VecDeque::new();
    let mut cpu_queue: VecDeque<Task> = VecDeque::new();
    let mut incoming_closed = false;

    loop {
        // Receive incoming tasks
        while let Ok(task) = incoming_receiver.try_recv() {
            match task.kind {
                crate::task::TaskKind::IO => io_queue.push_back(task),
                crate::task::TaskKind::CPU => cpu_queue.push_back(task),
            }
        }

        if !incoming_closed {
            match incoming_receiver.try_recv() {
                Ok(task) => {
                    match task.kind {
                        crate::task::TaskKind::IO => io_queue.push_back(task),
                        crate::task::TaskKind::CPU => cpu_queue.push_back(task),
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    incoming_closed = true;
                }
            }
        }

        let mut cpu = current_cpu.lock().unwrap();

        // 🔥 PRIORITY LOGIC
        if let Some(task) = io_queue.front() {
            if *cpu + task.cpu_cost <= 100 {
                let task = io_queue.pop_front().unwrap();
                *cpu += task.cpu_cost;

                println!(
                    "[OPT] Sending IO task {} | CPU {}%",
                    task.id, *cpu
                );

                worker_sender.send(task).unwrap();
            }
        } else if let Some(task) = cpu_queue.front() {
            if *cpu + task.cpu_cost <= 100 {
                let task = cpu_queue.pop_front().unwrap();
                *cpu += task.cpu_cost;

                println!(
                    "[OPT] Sending CPU task {} | CPU {}%",
                    task.id, *cpu
                );

                worker_sender.send(task).unwrap();
            }
        }

        drop(cpu);

        if incoming_closed && io_queue.is_empty() && cpu_queue.is_empty() {
            println!("[OPT] Dispatcher shutting down");
            break;
        }

        thread::sleep(Duration::from_millis(1));
    }
}