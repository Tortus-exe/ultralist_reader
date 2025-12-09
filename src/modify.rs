use uuid::Uuid;
use crate::serde_date_time::SerdeDateTime;
use crate::serde_date::SerdeDate;
use crate::Todo;
use crate::AppError;

fn get_contexts_and_projects(sub: &str) -> (Vec<String>, Vec<String>) {
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

const COMPLETED_STATUS: &str = "completed";
pub fn complete(todos: &mut Vec<Todo>, id: u64, set: bool) -> Result<(), AppError> {
    let todo: &mut Todo = find_todo_mut(todos, id)?;
    if set {
        todo.status = COMPLETED_STATUS.to_string();
        todo.completed_date = SerdeDateTime::now();
    } else {
        todo.status = "".to_string();
        todo.completed_date = SerdeDateTime::new_empty();
    }
    todo.completed = set;
    Ok(())
}

pub fn prioritize(todos: &mut Vec<Todo>, id: u64, set: bool) -> Result<(), AppError> {
    let todo: &mut Todo = find_todo_mut(todos, id)?;
    todo.is_priority = set;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Todo, SerdeDate, SerdeDateTime};

    fn gen_serdedate() -> SerdeDate {
        let serdedate = SerdeDate::try_from(Some("nov28".to_string()));
        assert!(serdedate.is_ok());
        serdedate.unwrap()
    }

    fn gen_todo() -> Vec<Todo> {
        let serdedate = gen_serdedate();

        vec![
            Todo {
                id: 0,
                uuid: "".to_string(),
                subject: "this is the subject".to_string(),
                projects: vec![],
                contexts: vec![],
                due: serdedate,
                completed: false,
                completed_date: SerdeDateTime::new_empty(),
                status: "".to_string(),
                archived: false,
                is_priority: false,
                notes: None,
                recur: "".to_string(),
                recur_until: "".to_string(),
                prev_recur_todo_uuid: "".to_string()
            }
        ]
    }

    #[test]
    fn test_add() {
        let mut todo: Vec<Todo> = vec![];

        add(&mut todo, "this is the subject".to_string(), gen_serdedate(), None);
        assert!(!todo[0].uuid.is_empty());
        todo[0].uuid = "".to_string();

        let todo_check: Vec<Todo> = gen_todo();
        assert_eq!(todo, todo_check);
    }

    #[test]
    fn test_status() {
        let mut todo = gen_todo();

        let r = status(&mut todo, 0, "GOOD".to_string());

        assert!(r.is_ok());
        assert_eq!(todo[0].status, "GOOD");
    }

    #[test]
    fn test_status_nonexistent() {
        let mut todo: Vec<Todo> = vec![];

        let r = status(&mut todo, 0, "GOOD".to_string());

        assert!(r.is_err());
        assert_eq!(r, Err(AppError::IdNotFoundError(0)));
    }

    #[test]
    fn test_complete() {
        let mut todo: Vec<Todo> = gen_todo();

        let r = complete(&mut todo, 0, true);

        assert!(r.is_ok());
        assert_eq!(todo[0].status, COMPLETED_STATUS);
        assert_eq!(todo[0].completed, true);
        assert!(todo[0].completed_date != SerdeDateTime::new_empty());
    }

    #[test]
    fn test_complete_nonexistent() {
        let mut todo: Vec<Todo> = vec![];

        let r = complete(&mut todo, 0, true);

        assert!(r.is_err());
        assert_eq!(r, Err(AppError::IdNotFoundError(0)));
    }

    #[test]
    fn test_complete_already_completed() {
        let mut todo: Vec<Todo> = gen_todo();

        todo[0].completed = true;
        todo[0].status = COMPLETED_STATUS.to_string();
        todo[0].completed_date = SerdeDateTime::now();

        let r = complete(&mut todo, 0, true);

        assert!(r.is_ok());
        assert_eq!(todo[0].status, COMPLETED_STATUS);
        assert_eq!(todo[0].completed, true);
        assert!(todo[0].completed_date != SerdeDateTime::new_empty());
    }

    #[test]
    fn test_uncomplete() {
        let mut todo: Vec<Todo> = gen_todo();

        todo[0].completed = true;
        todo[0].status = COMPLETED_STATUS.to_string();
        todo[0].completed_date = SerdeDateTime::now();

        let r = complete(&mut todo, 0, false);

        assert!(r.is_ok());
        assert_eq!(todo[0].status, "");
        assert_eq!(todo[0].completed, false);
        assert_eq!(todo[0].completed_date, SerdeDateTime::new_empty());
    }

    #[test]
    fn test_delete() {
        let mut todo: Vec<Todo> = gen_todo();

        let r = delete(&mut todo, 0);

        assert!(r.is_ok());
        assert!(todo.is_empty());
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut todo: Vec<Todo> = gen_todo();

        let r = delete(&mut todo, 1);

        assert!(r.is_err());
        assert_eq!(r, Err(AppError::IdNotFoundError(1)));
    }

    #[test]
    fn test_prioritize() {
        let mut todo: Vec<Todo> = gen_todo();

        let r = prioritize(&mut todo, 0, true);

        assert!(r.is_ok());
        assert_eq!(todo[0].is_priority, true);
    }

    #[test]
    fn test_prioritize_nonexistent() {
        let mut todo: Vec<Todo> = gen_todo();

        let r = prioritize(&mut todo, 1, true);

        assert!(r.is_err());
        assert_eq!(r, Err(AppError::IdNotFoundError(1)));
    }

    #[test]
    fn test_edit() {
        let mut todo: Vec<Todo> = gen_todo();

        let new_subj = "this is new subject";
        let r = edit(&mut todo, 0, new_subj.to_string(), SerdeDate::try_from(None).unwrap(), None);

        assert!(r.is_ok());
        assert_eq!(todo, vec![
            Todo {
                id: 0,
                uuid: "".to_string(),
                subject: new_subj.to_string(),
                projects: vec![],
                contexts: vec![],
                due: gen_serdedate(),
                completed: false,
                completed_date: SerdeDateTime::new_empty(),
                status: "".to_string(),
                archived: false,
                is_priority: false,
                notes: None,
                recur: "".to_string(),
                recur_until: "".to_string(),
                prev_recur_todo_uuid: "".to_string()
            }
        ]);
    }
}
