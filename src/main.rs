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

// const TODOS_FILENAME: &str = "/home/tortus/.todos.json";
const TODOS_FILENAME: &str = "output.json";

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

fn get_contexts_and_projects(sub: &String) -> (Vec<String>, Vec<String>) {
    let mut ctx = Vec::new();
    let mut projs = Vec::new();
    sub.split_whitespace().for_each(|word: &str| {
        match word.chars().nth(0) {
            Some('+') => {
                let chs: String = word.chars().skip(1).collect();
                if !chs.is_empty() {
                    projs.push(chs);
                }
            },
            Some('@') => {
                let chs: String = word.chars().skip(1).collect();
                if !chs.is_empty() {
                    ctx.push(chs);
                }
            },
            _ => ()
        };
    });
    (ctx, projs)
}

fn find_new_id(todos: &Vec<Todo>) -> u64 {
    let mut found: Vec<bool> = vec![false; todos.len()];
    todos.into_iter().for_each(|td| {
        if (td.id as usize) < found.len() {
            found[(td.id-1) as usize] = true;
        }
    });
    let idx = found.iter().position(|n| !*n);
    (match idx {
        Some(i) => i+1,
        None => todos.len()
    }) as u64
}

fn add(todos: &mut Vec<Todo>, sub: String, due: SerdeDate, _recur: Option<String>) {
    let (ctx, projs) = get_contexts_and_projects(&sub);
    let uuid = Uuid::new_v4();
    // dbg!(ctx);
    // dbg!(projs);
    // dbg!(sub);
    // dbg!(due);
    // println!("subject: {}, due: {}", sub, due);
    // dbg!(find_new_id(todos));
    let todo_to_add = Todo {
        id: find_new_id(todos),
        uuid: uuid.to_string(),
        subject: sub,
        projects: projs,
        contexts: ctx,
        due: due,
        completed: false,
        completed_date: SerdeDateTime::new_empty(),
        status: "".to_string(),
        archived: false,
        is_priority: false,
        notes: None,
        recur: "".to_string(),
        recur_until: "".to_string(),
        prev_recur_todo_uuid: "".to_string(),
    };
    todos.push(todo_to_add);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let todos_raw = fs::read_to_string(TODOS_FILENAME)?; // find some way to get the
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
