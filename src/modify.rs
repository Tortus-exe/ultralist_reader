use uuid::Uuid;
use crate::serde_date_time::SerdeDateTime;
use crate::serde_date::SerdeDate;
use crate::Todo;
use crate::AppError;

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

pub fn add(todos: &mut Vec<Todo>, sub: String, due: SerdeDate, recur: Option<String>) {
    let (ctx, projs) = get_contexts_and_projects(&sub);
    let uuid = Uuid::new_v4();
    let id = find_new_id(todos);
    let todo_to_add = Todo {
        id: id,
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
        recur: recur.unwrap_or("".to_string()),
        recur_until: "".to_string(),
        prev_recur_todo_uuid: "".to_string(),
    };
    todos.push(todo_to_add);
    println!("Todo {} added.", id);
}

pub fn find_todo_index(todos: &Vec<Todo>, id: u64) -> Result<usize, AppError> {
    if let Some(i) = todos.iter().position(|t| t.id == id) {
        return Ok(i);
    }
    Err(AppError::IdNotFoundError(id))
}

pub fn find_todo_mut(todos: &mut Vec<Todo>, id: u64) -> Result<&mut Todo, AppError> {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        return Ok(todo);
    }
    Err(AppError::IdNotFoundError(id))
}

pub fn edit(todos: &mut Vec<Todo>, id: u64, sub: String, due: SerdeDate, recur: Option<String>) -> Result<(), AppError> {
    let todo: &mut Todo = find_todo_mut(todos, id)?;
    if due.is_some() {
        todo.due = due;
    }
    if let Some(recurrance) = recur {
        todo.recur = recurrance;
    }
    todo.subject = sub;
    Ok(())
}

pub fn delete(todos: &mut Vec<Todo>, id: u64) -> Result<(), AppError> {
    let i: usize = find_todo_index(todos, id)?;
    todos.remove(i);
    return Ok(());
}

pub fn status(todos: &mut Vec<Todo>, id: u64, stat: String) -> Result<(), AppError> {
    let todo: &mut Todo = find_todo_mut(todos, id)?;
    todo.status = stat;
    return Ok(());
}
