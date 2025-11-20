pub mod serde_date_time;

use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use itertools::Itertools;
use std::fs;
use std::error::Error;
use std::iter;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tabled::{Table, settings::{style::Style, Color, themes::Colorization}, Tabled};
use crate::serde_date_time::SerdeDateTime;

#[derive(Subcommand, Debug)]
enum Command {
    List {
        #[arg(short, long)]
        group: Option<GroupOption>
    },
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum GroupOption {
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

#[derive(Tabled)]
struct TodoDisplay<'a> {
    id: u64,
    #[tabled(display("display_completed"))]
    completed: bool,
    status: &'a String,
    subject: String
}

fn colorize_subject(k: &String) -> String {
    k.split_whitespace().map(|word: &str| -> String {
        match word.chars().nth(0) {
            Some('+') => word.purple().to_string(),
            Some('@') => word.green().to_string(),
            _ => word.to_string()
        }
    }).join(" ")
}

fn display_completed(&val: &bool) -> String {
    format!("[{}]", if val {"x"} else {" "})
}

const FULL_GROUP_LABEL: &str = "All";
const NO_PROJECT_LABEL: &str = "No projects";
const NO_CONTEXT_LABEL: &str = "No contexts";
const NO_STATUS_LABEL: &str = "No status";
fn todo_grouping(todos: &Vec<Todo>, grouping: Option<GroupOption>) -> HashMap<&str, Vec<&Todo>> {
    match grouping {
        None => {
            let mut todo_pointer_vec = Vec::with_capacity(todos.len());
            todos.iter().for_each(|x| {todo_pointer_vec.push(x)});
            HashMap::from([(FULL_GROUP_LABEL, todo_pointer_vec)])
        },
        Some(GroupOption::Project) => {
            let mut groups: HashMap<&str, Vec<&Todo>> = HashMap::new();
            todos.iter().for_each(|todo| {
                let projects_list: Box<dyn Iterator<Item=&str>> = if todo.projects.is_empty() {
                    Box::new(iter::once(NO_PROJECT_LABEL))
                } else {
                    Box::new(todo.projects.iter().map(|proj| proj.as_ref()))
                };
                projects_list.for_each(|proj| {
                    groups.entry(proj)
                          .and_modify(|todos_list: &mut Vec<&Todo>| todos_list.push(todo))
                          .or_insert(vec![todo]);
                });
            });
            groups
        },
        Some(GroupOption::Context) => {
            let mut groups: HashMap<&str, Vec<&Todo>> = HashMap::new();
            todos.iter().for_each(|todo| {
                let contexts_list: Box<dyn Iterator<Item=&str>> = if todo.contexts.is_empty() {
                    Box::new(iter::once(NO_CONTEXT_LABEL))
                } else {
                    Box::new(todo.contexts.iter().map(|ctx| ctx.as_ref()))
                };
                contexts_list.for_each(|ctx| {
                    groups.entry(ctx)
                          .and_modify(|todos_list: &mut Vec<&Todo>| todos_list.push(todo))
                          .or_insert(vec![todo]);
                });
            });
            groups
        }, 
        Some(GroupOption::Status) => {
            let mut groups: HashMap<&str, Vec<&Todo>> = HashMap::new();
            todos.iter().for_each(|todo| {
                let stat: &str = if todo.status.is_empty() { NO_STATUS_LABEL } else { todo.status.as_ref() };
                groups.entry(stat)
                      .and_modify(|todos_list: &mut Vec<&Todo>| todos_list.push(todo))
                      .or_insert(vec![todo]);
            });
            groups
        }
    }
}

fn list(todos: &Vec<Todo>, grouping: Option<GroupOption>) -> () {
    let grouped_todo: HashMap<&str, Vec<&Todo>> = todo_grouping(todos, grouping);

    for (title, todo_group) in grouped_todo.iter() {
        let mut todos_display = Vec::new();
        for item in todo_group.iter() {
            if !item.archived {
                todos_display.push(TodoDisplay {
                    id: item.id,
                    completed: item.completed,
                    status: &item.status,
                    subject: colorize_subject(&item.subject)
                });
            }
        }

        let idcol = Color::FG_YELLOW;
        let complcol = Color::FG_BLUE;
        let statuscol = Color::FG_RED;
        let subjectcol = Color::FG_BRIGHT_WHITE;

        let mut table = Table::new(todos_display);
        table.with(Style::blank())
        .with(Colorization::columns([idcol, complcol, statuscol, subjectcol]));
        println!("{}:\n{}", title, table);
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
