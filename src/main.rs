use std::fs;
use std::error::Error;
use serde_json::{Value};
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    archived: bool, 
    completed: bool, 
    completed_date: String,
    contexts: Vec<String>,
    due: String,
    id: u64,
    is_priority: bool,
    notes: Option<Vec<String>>,
    prev_recur_todo_uuid: String,
    projects: Vec<String>,
    recur: String,
    recur_until: String,
    status: String,
    subject: String,
    uuid: String
}

fn main() -> Result<(), Box<dyn Error>> {
    let todos_raw = fs::read_to_string("/home/tortus/.todos.json")?;
    let r: Vec<Todo> = serde_json::from_str(&todos_raw)?;
    println!("{:?}", r);
    Ok(())
}
