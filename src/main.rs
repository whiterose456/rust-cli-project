use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};

const TODO_FILE: &str = "todo.json";

#[derive(Parser)]
#[command(author, version, about = "Simple CLI todo list", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo item
    Add {
        /// Todo description text
        description: String,
    },
    /// List all todo items
    List,
    /// Mark a todo item as done
    Done {
        /// ID of the item to complete
        id: usize,
    },
    /// Remove a todo item
    Remove {
        /// ID of the item to remove
        id: usize,
    },
    /// Clear all todo items
    Clear,
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { description } => {
            let id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
            tasks.push(Task {
                id,
                description,
                done: false,
            });
            save_tasks(&tasks);
            println!("Added todo #{}", id);
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No todo items found.");
                return;
            }
            println!("Todo list:");
            for task in &tasks {
                let status = if task.done { "[x]" } else { "[ ]" };
                println!("{} {}: {}", status, task.id, task.description);
            }
        }
        Commands::Done { id } => {
            if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                task.done = true;
                save_tasks(&tasks);
                println!("Marked todo #{} as done.", id);
            } else {
                println!("Todo item #{} not found.", id);
            }
        }
        Commands::Remove { id } => {
            let original_len = tasks.len();
            tasks.retain(|task| task.id != id);
            if tasks.len() < original_len {
                save_tasks(&tasks);
                println!("Removed todo #{}.", id);
            } else {
                println!("Todo item #{} not found.", id);
            }
        }
        Commands::Clear => {
            tasks.clear();
            save_tasks(&tasks);
            println!("Cleared all todo items.");
        }
    }
}

fn storage_path() -> PathBuf {
    PathBuf::from(TODO_FILE)
}

fn load_tasks() -> Vec<Task> {
    let path = storage_path();
    if !path.exists() {
        return Vec::new();
    }
    let data = fs::read_to_string(&path).unwrap_or_else(|_| String::new());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

fn save_tasks(tasks: &[Task]) {
    let path = storage_path();
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize todo data");
    let mut file = fs::File::create(&path).expect("Failed to open todo file");
    file.write_all(data.as_bytes())
        .expect("Failed to write todo file");
}
