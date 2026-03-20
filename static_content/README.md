# static_content

## Quick Start

```sh
cd web
npm i
./run
```

In another terminal:

```sh
cd app
./scripts/run
```

---

## Overview

`static_content` is a demonstration plugin for the Patoka distributed task orchestration framework. It showcases how to perform web scraping and data extraction using a combination of Rust (Actix actors) and JavaScript (headless browser automation).

---

## Architecture

- **Rust/Actix Backend**: Orchestrates tasks, manages state, and coordinates communication.
- **JavaScript Task Scripts**: Implement the logic for scraping and data extraction using a headless browser.
- **Patoka Integration**: Leverages Patoka's APIs for task management, messaging, and configuration.

---

## Main Components

### 1. Application Entry Point
- **File**: `app/src/main.rs`
- **Functionality**: Initializes the plugin and starts the main task logic if enabled in the environment.

### 2. Task Orchestration
- **File**: `app/src/tasks/mod.rs`
- **Functionality**: Checks if the plugin is enabled and starts the master task.

### 3. Master Task
- **File**: `app/src/tasks/master.rs`
- **Struct**: `MasterClient`
- **Purpose**:
	- Loads the target URL from configuration.
	- Starts the master JavaScript task (`js/master.js`) to extract a list of countries from a web page.
	- For each country, creates a subtask to extract its population.
	- Tracks subtask completion and stops when all are done.
- **Key Methods**:
	- `process_task_result`: Handles results from the JS master task, spawns subtasks.
	- `unsubscribe_from_subtasks`: Cleans up after all subtasks finish.

### 4. Subtask
- **File**: `app/src/tasks/subtask.rs`
- **Struct**: `SubtaskClient`
- **Purpose**:
	- Receives a country name and URL from the master task.
	- Runs the subtask JavaScript (`js/subtask.js`) to extract the population from the country's page.
	- Reports the result and stops.

### 5. JavaScript Task Logic
- **Files**: `app/src/tasks/js/master.js`, `app/src/tasks/js/subtask.js`
- **master.js**: Navigates to the main page, extracts country names and links.
- **subtask.js**: Navigates to each country page, extracts the population value.

---

## Data Flow

1. **Startup**: Rust app starts, checks if `static_content` is enabled.
2. **Master Task**: Loads the target URL, runs JS to extract countries.
3. **Subtask Creation**: For each country, a subtask is spawned.
4. **Subtask Execution**: Each subtask extracts the population in JS and reports back.
5. **Completion**: Master tracks subtask completion, cleans up, and exits.

---

## Key Rust Types

- `MasterTaskParams`: Parameters for the master task (target URL).
- `MasterTaskExecutionResult`: Result from master task (list of countries).
- `SubtaskParams`: Parameters for subtask (country name and URL).
- `SubtaskExecutionResult`: Result from subtask (population value).

---

## Extensibility

- **Add new scraping targets** by creating new Rust structs and JS scripts.
- **Change extraction logic** by editing the JS files.
- **Integrate with other Patoka plugins** for more complex workflows.

---

## Example Use Case

- **Web Scraping**: Extract a list of countries and their populations from a static website.
- **Template for Custom Scrapers**: Use as a starting point for building your own Patoka plugins for web data extraction.

---


## Web Folder

The `web` folder provides a static web application that serves as the data source for the plugin's scraping tasks. It is designed to present a fixed set of country data for extraction and demonstration purposes.

- **app.js**: An Express.js server that renders a list of countries and their populations. Each country has its own page.
- **views/index.pug**: Pug template for rendering the list of countries as links.
- **views/country.pug**: Pug template for rendering a table with the selected country's name and population.
- **run**: Script to start the web server.
- **Purpose**: The web app simulates a static data source for the plugin to extract and process, enabling reliable testing of static content extraction workflows.

## File Structure (Relevant Parts)

```
static_content/
├── app/
│   ├── src/
│   │   ├── main.rs
│   │   └── tasks/
│   │       ├── mod.rs
│   │       ├── master.rs
│   │       ├── subtask.rs
│   │       └── js/
│   │           ├── master.js
│   │           └── subtask.js
│   └── ...
├── web/
│   ├── app.js
│   ├── run
│   └── views/
│       ├── country.pug
│       └── index.pug
└── ...
```

---

## Summary

`static_content` demonstrates a modular, distributed web scraping system using Rust and JavaScript, suitable for scalable and scriptable data extraction workflows in the Patoka ecosystem.

