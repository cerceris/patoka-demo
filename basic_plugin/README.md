
# basic_plugin

## Quick Start

```sh
npm i
./scripts/run
```

---

## Overview

`basic_plugin` is a demonstration plugin for the Patoka distributed task orchestration framework. It combines a Rust backend (using Actix actors) with JavaScript-based task logic, illustrating how to build scalable, scriptable workflows.

---

## Architecture

- **Rust/Actix Backend**: Manages orchestration, state, and communication between tasks.
- **JavaScript Task Scripts**: Implement the actual logic for master and subtask operations.
- **Patoka Integration**: Uses Patoka's APIs for task management, messaging, and configuration.

---

## Main Components

### 1. Application Entry Point
- **File**: `src/main.rs`
- **Functionality**: Initializes the plugin and starts the main task logic if enabled in the environment.

### 2. Task Orchestration
- **File**: `src/tasks/mod.rs`
- **Functionality**: Checks if the plugin is enabled and starts the master task.

### 3. Master Task
- **File**: `src/tasks/master.rs`
- **Struct**: `MasterClient`
- **Purpose**:
    - Loads parameters (`number`, `max`) from configuration.
    - Starts the master JavaScript task (`js/master.js`) to generate random numbers.
    - For each generated number, creates a subtask.
    - Tracks subtask completion and stops when all are done.
- **Key Methods**:
    - `process_task_result`: Handles results from the JS master task, spawns subtasks.
    - `unsubscribe_from_subtasks`: Cleans up after all subtasks finish.

### 4. Subtask
- **File**: `src/tasks/subtask.rs`
- **Struct**: `SubtaskClient`
- **Purpose**:
    - Receives a number from the master task.
    - Runs the subtask JavaScript (`js/subtask.js`) to compute its square.
    - Reports the result and stops.

### 5. JavaScript Task Logic
- **Files**: `src/tasks/js/master.js`, `src/tasks/js/subtask.js`
- **master.js**: Generates random numbers using the `chance` library and sends them back to Rust.
- **subtask.js**: Computes the square of a given number and returns the result.

---

## Data Flow

1. **Startup**: Rust app starts, checks if `basic_plugin` is enabled.
2. **Master Task**: Loads parameters, runs JS to generate numbers.
3. **Subtask Creation**: For each number, a subtask is spawned.
4. **Subtask Execution**: Each subtask computes a square in JS and reports back.
5. **Completion**: Master tracks subtask completion, cleans up, and exits.

---

## Key Rust Types

- `MasterTaskParams`: Parameters for the master task (number of values, max value).
- `MasterTaskExecutionResult`: Result from master task (list of numbers).
- `SubtaskParams`: Parameter for subtask (single number).
- `SubtaskExecutionResult`: Result from subtask (square of number).

---

## Extensibility

- **Add new task types** by creating new Rust structs and JS scripts.
- **Change task logic** by editing the JS files.
- **Integrate with other Patoka plugins** for more complex workflows.

---

## Example Use Case

- **Distributed Computation**: Generate a set of random numbers, then process each independently (e.g., squaring, as in this demo).
- **Template for Custom Plugins**: Use as a starting point for building your own Patoka plugins with custom logic.

---

## File Structure (Relevant Parts)

```
basic_plugin/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── tasks/
│       ├── mod.rs
│       ├── master.rs
│       ├── subtask.rs
│       └── js/
│           ├── master.js
│           └── subtask.js
```

---

## Summary

`basic_plugin` demonstrates a modular, distributed task system using Rust and JavaScript, suitable for scalable and scriptable workflows in the Patoka ecosystem.
