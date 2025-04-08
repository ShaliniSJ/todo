use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod storage;
mod task;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple To-Do CLI", version = "1.0")]
struct Cli {
    #[arg(long)]
    path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Done { id: u32 },
}

fn list_tasks(tasks: &[task::Task]) {
    for task in tasks {
        let status = if task.is_done { "✅" } else { "⏳" };
        if !task.is_done {
            println!("{}", format!("{} [{}] {}", task.id, status, task.title));
        } else {
            println!(
                "{}",
                format!("{} [{}] {}", task.id, status, task.title).dimmed()
            );
        }
    }
}

const DEFAULT_PATH: &str = "{{give yours}}";

fn main() {
    let cli = Cli::parse();

    // Use provided path or fallback to default path
    let task_path: PathBuf = cli.path.unwrap_or_else(|| PathBuf::from(DEFAULT_PATH));
    println!("📁 Using task file at: {}", task_path.display());
    println!();
    let mut tasks = storage::load_tasks(&task_path);

    match cli.command {
        Commands::Add { task } => {
            let next_id = (tasks.len() as u32) + 1;
            let new_task = task::Task {
                id: next_id,
                title: task,
                is_done: false,
            };
            tasks.push(new_task);
            storage::save_tasks(&tasks, &task_path);
            println!("{}", "✅ Task added!".green());
        }

        Commands::List => {
            if tasks.is_empty() {
                println!("{}", "No tasks found.".dimmed());
            } else {
                println!("{}", "Tasks:".bold());
            }
        }

        Commands::Done { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.is_done = true;
                storage::save_tasks(&tasks, &task_path);
                println!("{}", "✔️ Task marked as done!".green());
            } else {
                println!("{}", format!("❌ Task with ID {} not found.", id).red());
            }
        }
    }
    println!();
    list_tasks(&tasks);
    println!();
    println!();
}
