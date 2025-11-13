pub mod serde_date_time;

use clap::{Parser, Subcommand};
use std::fs;
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::serde_date_time::SerdeDateTime;

#[derive(Subcommand, Debug)]
enum Command {
    List {
        #[arg(short, long)]
        group: Option<String>
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: u64,
    uuid: String,
    subject: String,
    projects: Vec<String>,
    contexts: Vec<String>,
    due: String,
    completed: bool, 
    completed_date: SerdeDateTime,
    status: String,
    archived: bool, 
    is_priority: bool,
    notes: Option<Vec<String>>,
    recur: String,
    recur_until: String,
    prev_recur_todo_uuid: String,
}

fn list(todos: &Vec<Todo>, _: Option<String>) -> () {
    for i in todos.iter() {
        if !i.archived {
            println!("{} {} {}", i.id, i.status, i.subject);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let todos_raw = fs::read_to_string("/home/tortus/.todos.json")?; // find some way to get the
                                                                     // home directory?
    let r: Vec<Todo> = serde_json::from_str(&todos_raw)?;
    // println!("{:?}", r);
    match args.command {
        Command::List { group: a } => list(&r, a),
    }

    let redone = serde_json::to_string(&r)?;
    fs::write("output.json", redone)?;
    Ok(())
}
