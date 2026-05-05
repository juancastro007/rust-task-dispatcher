# Concurrent Task Dispatcher in Rust — Design Report

## 1. Introduction

This project implements a concurrent task dispatcher in Rust that simulates how an operating system schedules and executes tasks under resource constraints. The system models both CPU-bound and IO-bound workloads and compares different scheduling strategies.

The goal of the project is to explore:
- concurrency using threads
- task scheduling policies
- resource management (CPU usage)
- performance measurement

---

## 2. System Architecture

The system is composed of the following components:

### Task Generator
Tasks are generated with the following attributes:
- ID
- arrival time
- type (IO or CPU)
- duration (200 ms)
- CPU cost (10% for IO, 35% for CPU)

Tasks arrive every 20 ms to simulate a real-time workload.

---

### Dispatcher (Scheduler)

The dispatcher is responsible for:
- receiving tasks
- placing them into queues
- assigning tasks to workers

Two scheduling policies are implemented:

#### FIFO Scheduler
- Uses a single queue
- Tasks are processed in arrival order
- No distinction between task types

#### Optimized Scheduler
- Uses two queues:
  - IO queue
  - CPU queue
- Prioritizes IO tasks when CPU usage is high
- Ensures total CPU usage does not exceed 100%

---

### Worker Pool

- The system uses 8 worker threads
- Each worker:
  - retrieves tasks from a shared queue
  - simulates execution using `sleep(200 ms)`
  - updates CPU usage and metrics

Workers terminate cleanly when all tasks are completed.

---

### Monitor Thread

- Runs independently from the worker pool
- Logs system state every 10 ms
- Tracks:
  - CPU usage
  - number of completed tasks
- Terminates when all tasks are completed and CPU usage returns to zero

---

### Metrics System

The system records:
- total tasks completed
- number of CPU tasks completed
- number of IO tasks completed
- average wait time
- average turnaround time
- total runtime

These metrics are used to compare scheduling strategies.

---

## 3. Concurrency Design

The system uses Rust’s standard concurrency primitives:

- `thread` for parallel execution
- `mpsc` channels for communication
- `Arc<Mutex<T>>` for shared state

### Channels
Channels are used to:
- send tasks from generator → dispatcher
- send tasks from dispatcher → workers

### Shared State
Shared variables include:
- current CPU usage
- metrics

These are protected using `Arc<Mutex<...>>` to ensure thread safety.

---

## 4. Scheduling Policies

### FIFO Scheduler

The FIFO scheduler:
- sends tasks in arrival order
- does not consider task type or CPU load beyond the 100% cap

Limitations:
- CPU-heavy tasks can delay IO tasks
- reduced responsiveness

---

### Optimized Scheduler

The optimized scheduler:
- separates tasks into IO and CPU queues
- prioritizes IO tasks when CPU usage is constrained

Benefits:
- faster completion of IO tasks
- improved system responsiveness
- better CPU utilization

Trade-offs:
- increased implementation complexity
- potential imbalance if CPU tasks are continuously delayed

---

## 5. Experiments

Two workloads were tested:

### Experiment A
- 700 IO tasks
- 300 CPU tasks

### Experiment B
- 800 IO tasks
- 200 CPU tasks

Each experiment was run using:
- FIFO scheduler
- Optimized scheduler

---

## 6. Results

Example results:
Total runtime: 47750 ms
Total tasks completed: 1000
CPU tasks completed: 300
IO tasks completed: 700
Average wait time: 5806.83 ms
Average turnaround time: 6007.04 ms


### Observations

- FIFO scheduling results in higher wait times when CPU tasks dominate
- Optimized scheduling reduces wait time for IO tasks
- CPU usage remains within the 100% constraint in both cases

---

## 7. Analysis

The optimized scheduler consistently performs better than FIFO in mixed workloads.

Key findings:
- IO tasks benefit significantly from prioritization due to lower CPU cost
- FIFO scheduling can lead to inefficient CPU usage when tasks are not balanced
- Optimized scheduling improves throughput and responsiveness

---

## 8. Challenges and Debugging

Several issues were encountered:

- Monitor thread not terminating correctly
  - Fixed by checking both CPU usage and task completion

- Incorrect CPU tracking
  - Resolved by synchronizing updates using `Mutex`

- Task arrival timing inconsistencies
  - Addressed using fixed sleep intervals

---

## 9. Limitations

- CPU usage is simulated rather than measured
- No real IO operations are performed
- No dynamic priority adjustment

---

## 10. Conclusion

This project demonstrates how scheduling policies and concurrency design affect system performance.

The optimized scheduler improves performance by:
- prioritizing low-cost tasks
- maintaining better resource utilization

The system provides a foundation for further extensions such as:
- priority scheduling
- work stealing
- dynamic load balancing

---

## 11. Tool Use Disclosure

AI tools were used to assist with:
- structuring the project
- debugging concurrency issues
- understanding scheduling strategies

All code and design decisions were reviewed and understood before final implementation.