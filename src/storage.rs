use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::task::Task;

pub fn load_tasks(path: &Path) -> Vec<Task> {
    if !path.exists() {
        return Vec::new(); // Return empty if file doesn't exist
    }

    let file = File::open(path).expect("Failed to open tasks file");
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
}

pub fn save_tasks(tasks: &Vec<Task>, path: &Path) {
    // Make sure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create task file directory");
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open or create tasks file");

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks).expect("Failed to write tasks to JSON");
}
