# 📝 To-Do CLI in Rust

A simple and fast command-line To-Do List tool built with Rust.  
Supports persistent file storage using JSON, colored output, and flexible path configuration.

---

## 🚀 Features

-   Add, list, mark tasks as done ✅
-   Clear all tasks with one command
-   File-based JSON storage
-   Customizable path for your task file
-   Colored CLI output

---

## 🛠 Installation

### 1. Clone the repo

```bash
git clone https://github.com/your-username/todo-cli-rust.git
cd todo-cli-rust
```

### 2. Build the release binary

```
cargo build --release
```

### 3. Add the binary to your system PATH (Windows)

-   Copy the binary from:

```
target/release/todo.exe
```

-   To a folder included in your PATH, or add a new folder (like C:\CLI_Tools) and add that to your PATH environment variable.

-   Run the tool in a terminal after adding to PATH or from target/release/todo.exe

## 📦 Usage

-   Add a task

```
todo add "Learn Rust"
```

-   List tasks

```
todo list
```

-   Mark a task as done

```
todo done <task_id>
```

-   Use a custom path

    -   By default, tasks are saved to the path which the main.rs has

    -   You can override it like this:

    ```
    todo --path "C:/Users/Shalini/Desktop/my_tasks.json" add "Another task"
    ```

## 🧱 Project Structure

```
src/
├── main.rs # CLI entry point
├── task.rs # Task struct (id, title, is_done)
└── storage.rs # File read/write logic
```

### 📚 Dependencies

    clap – CLI argument parsing

    serde – Serialization

    colored – Colored terminal output

Install them using Cargo.toml.
💡 Ideas to Extend

    Add deadlines or priorities

    Add fuzzy search

    Sync with cloud storage

    Export/import tasks

    GUI frontend using Tauri or egui

✨ Credits

Built with 💙 by [Shalini] using Rust 🦀

Happy shipping, CLI wizard 🪄🚀

---
