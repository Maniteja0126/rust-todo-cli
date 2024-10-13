use clap::{Arg, Command};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    task: String,
    completed: bool,
}

const FILE_PATH: &str = "task.json";

fn load_task() -> Vec<Task> {
    if !Path::new(FILE_PATH).exists() {
        return Vec::new();
    }

    let data = fs::read_to_string(FILE_PATH).expect("Unable to read task.json");
    serde_json::from_str(&data).unwrap_or_else(|_| {
        eprintln!(
            "{}",
            "Failed to parse tasks. Initializing an empty list.".red()
        );
        Vec::new()
    })
}

fn save_task(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).expect("Unable to serialize tasks");
    let mut file = File::create(FILE_PATH).expect("Unable to create task.json");
    file.write_all(json.as_bytes())
        .expect("Unable to write to task.json");
}

fn main() {
    let matches = Command::new("Task Manager")
        .version("1.0")
        .author("Mani Teja")
        .about("Manage your tasks")
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(Arg::new("task").required(true)),
        )
        .subcommand(
            Command::new("update")
                .about("Update existing task")
                .arg(Arg::new("index").required(true))
                .arg(Arg::new("new_task").required(true)),
        )
        .subcommand(
            Command::new("complete")
                .about("Mark as completed")
                .arg(Arg::new("index").required(true)),
        )
        .subcommand(Command::new("list").about("List all tasks"))
        .subcommand(
            Command::new("delete")
                .about("Delete a task")
                .arg(Arg::new("index").required(true)),
        )
        .get_matches();
    if let Some(add_matches) = matches.subcommand_matches("add") {
        let task = add_matches.get_one::<String>("task").unwrap();
        let mut tasks = load_task();
        tasks.push(Task {
            task: task.to_string(),
            completed: false,
        });
        save_task(&tasks);
        println!("{}", "Task added successfully!".green());
    }
    if let Some(update_matches) = matches.subcommand_matches("update") {
        let index: usize = update_matches
            .get_one::<String>("index")
            .unwrap()
            .parse()
            .unwrap();
        let new_task = update_matches.get_one::<String>("new_task").unwrap();
        let mut tasks = load_task();

        if index < 1 || index > tasks.len() {
            println!("{}", "Invalid task number!".red());
        } else {
            tasks[index - 1].task = new_task.to_string();
            save_task(&tasks);
            println!("{}", "Task updated successfully!".green());
        }
    }

    if let Some(complete_matches) = matches.subcommand_matches("complete") {
        let index: usize = complete_matches
            .get_one::<String>("index")
            .unwrap()
            .parse()
            .unwrap();
        let mut tasks = load_task();
        if index < 1 || index > tasks.len() {
            println!("{}", "Invalid task number!".red());
        } else {
            tasks[index - 1].completed = true;
            save_task(&tasks);
            println!("{}", "Task marked as completed!".green());
        }
    }

    if matches.subcommand_matches("list").is_some() {
        let tasks = load_task();
        println!("{}", "Tasks:".blue());
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.completed {
                "[Done]".green()
            } else {
                "[Pending]".red()
            };
            println!("{}: {} {}", i + 1, task.task, status);
        }
    }

    if let Some(delete_matches) = matches.subcommand_matches("delete") {
        let index: usize = delete_matches
            .get_one::<String>("index")
            .unwrap()
            .parse()
            .unwrap();
        let mut tasks = load_task();
        if index < 1 || index > tasks.len() {
            println!("{}", "Invalid task number!".red());
        } else {
            tasks.remove(index - 1);
            save_task(&tasks);
            println!("{}", "Task deleted successfully!".green());
        }
    }
}
