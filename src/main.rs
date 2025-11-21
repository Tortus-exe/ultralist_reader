pub mod serde_date_time;
pub mod serde_date;
pub mod list;

use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::error::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::serde_date_time::SerdeDateTime;
use crate::serde_date::SerdeDate;
use crate::list::list;

#[derive(Subcommand, Debug)]
enum Command {
    List {
        #[arg(short, long)]
        group: Option<GroupOption>
    },
    Add {
        #[arg(short, long)]
        due: Option<String>,
        #[arg(short, long)]
        recur: Option<String>,
        subject: Vec<String>,
    }
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum GroupOption {
    Project,
    Context,
    Status
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    id: u64,
    uuid: String,
    subject: String,
    projects: Vec<String>,
    contexts: Vec<String>,
    due: SerdeDate,
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

fn add(_todos: &mut Vec<Todo>, sub: String, due: SerdeDate, _recur: Option<String>) {
    println!("subject: {}, due: {}", sub, due);
    todo!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let todos_raw = fs::read_to_string("/home/tortus/.todos.json")?; // find some way to get the
                                                                     // home directory?
    let mut r: Vec<Todo> = serde_json::from_str(&todos_raw)?;
    // println!("{:?}", r);
    match args.command {
        Command::List { group: a } => list(&r, a),
        Command::Add { due: d, recur: rc, subject: s } => add(&mut r, s.join(" "), SerdeDate::try_from(d)?, rc),
    }

    let redone = serde_json::to_string(&r)?;
    fs::write("output.json", redone)?;
    Ok(())
}
