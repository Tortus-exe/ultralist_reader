use std::fs;
use std::error::Error;
use serde_json::{Value};
use std::fmt;

#[derive(Debug, Clone)]
struct TodoParseError {src: u8}

impl fmt::Display for TodoParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse Error in Todo!")
    }
}
impl Error for TodoParseError {}

#[derive(Debug, Clone)]
struct Todo {
    archived: bool, 
    completed: bool, 
    completed_date: String,
    contexts: Vec<String>,
    due: String,
    id: u64,
    is_priority: bool,
    notes: Vec<String>,
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
    let v: Value = serde_json::from_str(&todos_raw)?;
    // println!("{}", type_of(&v[0]["notes"]));
    println!("{:?}", to_todos_list(&v)?);
    Ok(())
}

fn to_todos_list(input_json: &Value) -> Result<Vec<Todo>, Box<dyn Error>> {
    let inner_array = input_json.as_array();
    if let Some(x) = inner_array {
        Ok(x.into_iter().map(|todo_value| -> Result<Todo, TodoParseError> {
            let empty_vec: Vec<Value> = vec![];
            let context_arr: &Vec<Value> = if todo_value["contexts"].is_null() {
                &empty_vec
            } else {
                todo_value["contexts"].as_array().ok_or(TodoParseError {src: 0})?
            };
            let notes_arr: &Vec<Value> = if todo_value["notes"].is_null() {
                &empty_vec
            } else {
                todo_value["notes"].as_array().ok_or(TodoParseError {src: 1})?
            };
            let projects_arr: &Vec<Value> = if todo_value["projects"].is_null() {
                &empty_vec
            } else {
                todo_value["projects"].as_array().ok_or(TodoParseError {src: 2})?
            };
            Ok(Todo {
                archived: todo_value["archived"].as_bool().ok_or(TodoParseError {src: 3})?,
                completed: todo_value["completed"].as_bool().ok_or(TodoParseError {src: 4})?,
                completed_date: todo_value["completed_date"].as_str().ok_or(TodoParseError {src: 5}).map(|i|{String::from(i)})?,
                contexts: context_arr.into_iter().map(|val| -> Result<String, TodoParseError> {val.as_str().ok_or(TodoParseError {src: 6}).map(|i|{String::from(i)})}).collect::<Result<Vec<String>, TodoParseError>>()?,
                due: todo_value["due"].as_str().ok_or(TodoParseError {src: 7}).map(|i|{String::from(i)})?,
                id: todo_value["id"].as_u64().ok_or(TodoParseError {src: 8})?,
                is_priority: todo_value["is_priority"].as_bool().ok_or(TodoParseError {src: 9})?,
                notes: notes_arr.into_iter().map(|val| -> Result<String, TodoParseError> {val.as_str().ok_or(TodoParseError {src: 10}).map(|i|{String::from(i)})}).collect::<Result<Vec<String>, TodoParseError>>()?,
                prev_recur_todo_uuid: todo_value["prev_recur_todo_uuid"].as_str().ok_or(TodoParseError {src: 11}).map(|i|{String::from(i)})?,
                projects: projects_arr.into_iter().map(|val| -> Result<String, TodoParseError> {val.as_str().ok_or(TodoParseError {src: 12}).map(|i|{String::from(i)})}).collect::<Result<Vec<String>, TodoParseError>>()?,
                recur: todo_value["recur"].as_str().ok_or(TodoParseError {src: 13}).map(|i|{String::from(i)})?,
                recur_until: todo_value["recur_until"].as_str().ok_or(TodoParseError {src: 14}).map(|i|{String::from(i)})?,
                status: todo_value["status"].as_str().ok_or(TodoParseError {src: 15}).map(|i|{String::from(i)})?,
                subject: todo_value["subject"].as_str().ok_or(TodoParseError {src: 16}).map(|i|{String::from(i)})?,
                uuid: todo_value["uuid"].as_str().ok_or(TodoParseError {src: 17}).map(|i|{String::from(i)})?,
            })
        }).collect::<Result<Vec<Todo>, TodoParseError>>()?)
    } else {
        Err(Box::new(TodoParseError {src: 18}))
    }
}
