use rand::Rng;
use std::time::Instant;

#[derive(Clone)]
pub enum TaskKind {
    IO,
    CPU,
}

#[derive(Clone)]
pub struct Task {
    pub id: usize,
    pub arrival_time: Instant,
    pub kind: TaskKind,
    pub duration_ms: u64,
    pub cpu_cost: u32,
}

// Generate tasks with given IO ratio
pub fn generate_exact_tasks(io_count: usize, cpu_count: usize) -> Vec<Task> {
    let mut tasks = Vec::new();

    for i in 0..io_count {
        tasks.push(Task {
            id: i,
            arrival_time: Instant::now(),
            kind: TaskKind::IO,
            duration_ms: 200,
            cpu_cost: 10,
        });
    }

    for i in io_count..(io_count + cpu_count) {
        tasks.push(Task {
            id: i,
            arrival_time: Instant::now(),
            kind: TaskKind::CPU,
            duration_ms: 200,
            cpu_cost: 35,
        });
    }

    tasks
}