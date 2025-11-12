pub mod serde_date_time;

use std::fs;
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::serde_date_time::SerdeDateTime;

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

fn main() -> Result<(), Box<dyn Error>> {
    let todos_raw = fs::read_to_string("/home/tortus/.todos.json")?;
    let r: Vec<Todo> = serde_json::from_str(&todos_raw)?;
    println!("{:?}", r);
    let redone = serde_json::to_string(&r)?;
    fs::write("output.json", redone)?;
    Ok(())
}
