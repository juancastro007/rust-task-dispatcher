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
pub fn generate_tasks(total: usize, io_ratio: f64) -> Vec<Task> {
    let mut rng = rand::thread_rng();
    let mut tasks = Vec::new();

    for i in 0..total {
        let is_io = rng.gen_bool(io_ratio);

        let (kind, cpu_cost) = if is_io {
            (TaskKind::IO, 10)
        } else {
            (TaskKind::CPU, 35)
        };

        tasks.push(Task {
            id: i,
            arrival_time: Instant::now(),
            kind,
            duration_ms: 200,
            cpu_cost,
        });
    }

    tasks
}