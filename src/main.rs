pub mod serde_date_time;
pub mod serde_date;
pub mod list;
pub mod modify;
pub mod notes;

use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::error::Error;
use std::fmt;
use serde::{Deserialize, Serialize};
use crate::serde_date_time::SerdeDateTime;
use crate::serde_date::SerdeDate;
use crate::list::list;
use crate::modify::{add, edit, delete, status, complete, prioritize};
use crate::notes::{add_note, edit_note, delete_note};

// const TODOS_FILENAME: &str = "/home/tortus/.todos.json";
const TODOS_FILENAME: &str = "output.json";

#[derive(Debug)]
pub enum AppError {
    IdNotFoundError(u64),
    NoteNotFoundError(u64, usize)
}
impl Error for AppError {}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IdNotFoundError(i) => write!(f, "ID not found: {}", i),
            AppError::NoteNotFoundError(j, i) => write!(f, "Note number {} not found on todo number {}!", i, j),
        }
    }
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(alias("ls"))]
    #[clap(alias("l"))]
    List {
        #[arg(short, long)]
        group: Option<GroupOption>,
        #[arg(short, long, default_value_t=false)]
        notes: bool,
    },
    #[clap(alias("a"))]
    Add {
        #[arg(short, long)]
        due: Option<String>,
        #[arg(short, long)]
        recur: Option<String>,
        subject: Vec<String>,
    },
    #[clap(alias("e"))]
    Edit {
        id: u64,
        #[arg(short, long)]
        due: Option<String>,
        #[arg(short, long)]
        recur: Option<String>,
        subject: Vec<String>,
    },
    #[clap(alias("d"))]
    Delete {
        id: u64
    },
    #[clap(alias("s"))]
    Status {
        id: u64,
        stat: String
    },
    #[clap(alias("an"))]
    AddNote {
        id: u64,
        note: String
    },
    #[clap(alias("en"))]
    EditNote {
        id: u64,
        index: usize,
        note: String
    },
    #[clap(alias("dn"))]
    DeleteNote {
        id: u64,
        index: usize
    },
    #[clap(alias("c"))]
    Complete {
        id: u64
    },
    #[clap(alias("uc"))]
    Uncomplete {
        id: u64
    },
    #[clap(alias("p"))]
    Prioritize {
        id: u64
    },
    #[clap(alias("up"))]
    Unprioritize {
        id: u64
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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let todos_raw = fs::read_to_string(TODOS_FILENAME)?; // find some way to get the
                                                                     // home directory?
    let mut r: Vec<Todo> = serde_json::from_str(&todos_raw)?;
    // println!("{:?}", r);
    match args.command {
        Command::List { group: a, notes: b } => list(&r, a, b),
        Command::Add { due: d, recur: rc, subject: s } => add(&mut r, s.join(" "), SerdeDate::try_from(d)?, rc),
        Command::Edit { id: i, due: d, recur: rc, subject: s } => edit(&mut r, i, s.join(" "), SerdeDate::try_from(d)?, rc)?,
        Command::Delete { id: i } => delete(&mut r, i)?,
        Command::Status { id: i, stat: s } => status(&mut r, i, s)?,
        Command::AddNote { id: i, note: n } => add_note(&mut r, i, n)?,
        Command::EditNote { id: i, index: x, note: n } => edit_note(&mut r, i, x, n)?,
        Command::DeleteNote { id: i, index: x } => delete_note(&mut r, i, x)?,
        Command::Complete { id: i } => complete(&mut r, i, true)?,
        Command::Uncomplete { id: i } => complete(&mut r, i, false)?,
        Command::Prioritize { id: i } => prioritize(&mut r, i, true)?,
        Command::Unprioritize { id: i } => prioritize(&mut r, i, false)?
    }

    let redone = serde_json::to_string(&r)?;
    fs::write("output.json", redone)?;
    Ok(())
}
