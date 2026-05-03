use crate::task::TaskKind;

pub struct Metrics {
    pub completed_tasks: usize,
    pub cpu_tasks_completed: usize,
    pub io_tasks_completed: usize,
    pub total_wait_ms: u128,
    pub total_turnaround_ms: u128,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            completed_tasks: 0,
            cpu_tasks_completed: 0,
            io_tasks_completed: 0,
            total_wait_ms: 0,
            total_turnaround_ms: 0,
        }
    }

    pub fn record_task(&mut self, kind: &TaskKind, wait_ms: u128, turnaround_ms: u128) {
        self.completed_tasks += 1;
        self.total_wait_ms += wait_ms;
        self.total_turnaround_ms += turnaround_ms;

        match kind {
            TaskKind::CPU => self.cpu_tasks_completed += 1,
            TaskKind::IO => self.io_tasks_completed += 1,
        }
    }

    pub fn print_summary(&self, total_runtime_ms: u128) {
        println!("\n========== Simulation Summary ==========");
        println!("Total runtime: {} ms", total_runtime_ms);
        println!("Total tasks completed: {}", self.completed_tasks);
        println!("CPU tasks completed: {}", self.cpu_tasks_completed);
        println!("IO tasks completed: {}", self.io_tasks_completed);

        if self.completed_tasks > 0 {
            println!(
                "Average wait time: {:.2} ms",
                self.total_wait_ms as f64 / self.completed_tasks as f64
            );

            println!(
                "Average turnaround time: {:.2} ms",
                self.total_turnaround_ms as f64 / self.completed_tasks as f64
            );
        }

        println!("========================================");
    }
}