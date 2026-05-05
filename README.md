# Concurrent Task Dispatcher in Rust

## 📌 Project Overview

This project implements a **concurrent task dispatcher** in Rust that simulates how an operating system or service scheduler manages incoming work.

Tasks arrive over time and are dispatched to a **bounded worker pool (8 threads)**. The system supports multiple scheduling strategies and measures performance using system metrics.

---

## ⚙️ How to Build and Run

### Build
```bash

cd /workspaces/rust-task-dispatcher/rust-task-dispatcher

cargo build
cargo run -- 700      # FIFO (700 IO / 300 CPU)
cargo run -- 800      # FIFO (800 IO / 200 CPU)

cargo run -- opt700   # Optimized scheduler (700 IO / 300 CPU)
cargo run -- opt800   # Optimized scheduler (800 IO / 200 CPU)

cargo run -- both     # Runs FIFO + optimized comparison

---

## 🧠 System Architecture

The system consists of:

- Task Generator: creates tasks at 20 ms intervals
- Dispatcher: assigns tasks to workers
- Worker Pool: 8 threads executing tasks
- Monitor: logs CPU usage every 10 ms
- Metrics: tracks performance statistics

---

## ⚙️ Scheduling Policies

### FIFO Scheduler
- Single queue
- Processes tasks in arrival order
- Does not distinguish between IO and CPU tasks

### Optimized Scheduler
- Uses two queues:
  - IO queue
  - CPU queue
- Prioritizes IO tasks when CPU usage is high
- Keeps CPU usage under 100%

---

## 📊 Workloads

Two workloads were tested:

- 700 IO / 300 CPU tasks
- 800 IO / 200 CPU tasks

Each task:
- runs for 200 ms
- IO uses 10% CPU
- CPU uses 35% CPU

---

## 📈 Example Results

========== Simulation Summary ==========
Total runtime: 47774 ms
Total tasks completed: 1000
CPU tasks completed: 300
IO tasks completed: 700
Average wait time: 5778.77 ms
Average turnaround time: 5979.00 ms
========================================
Finished simulation: FIFO 700 IO / 300 CPU

========== Simulation Summary ==========
Total runtime: 40209 ms
Total tasks completed: 1000
CPU tasks completed: 200
IO tasks completed: 800
Average wait time: 3870.37 ms
Average turnaround time: 4070.64 ms
========================================
Finished simulation: FIFO 800 IO / 200 CPU


---

## ⚖️ Trade-offs

FIFO:
- Simple to implement
- Can delay IO tasks if CPU tasks dominate

Optimized:
- Improves responsiveness for IO tasks
- Better CPU utilization
- Slightly more complex design

---

## 🤖 Tool Use Disclosure

AI tools were used to assist with:
- project structure design
- debugging Rust concurrency issues
- understanding scheduling strategies

All code and concepts were reviewed and understood before final implementation.