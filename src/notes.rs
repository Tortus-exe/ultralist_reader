use crate::{Todo, AppError, modify::{find_todo_mut}};

pub fn add_note(todos: &mut Vec<Todo>, id: u64, note: String) -> Result<(), AppError> {
    let todo: &mut Todo = find_todo_mut(todos, id)?;
    match todo.notes.as_mut() {
        Some(notes) => {
            notes.push(note);
            println!("len: {}", notes.len());
        }
        None => {todo.notes = Some(vec![note]);}
    }
    Ok(())
}

pub fn edit_note(todos: &mut Vec<Todo>, id: u64, index: usize, note: String) -> Result<(), AppError> {
    let todo: &mut Todo = find_todo_mut(todos, id)?;
    let Some(notes) = &mut todo.notes else {return Err(AppError::NoteNotFoundError(id, index))};
    if index < notes.len() {
        notes[index] = note;
        Ok(())
    } else {
        Err(AppError::NoteNotFoundError(id, index))
    }
}

pub fn delete_note(todos: &mut Vec<Todo>, id: u64, index: usize) -> Result<(), AppError> {
    let todo: &mut Todo = find_todo_mut(todos, id)?;
    let Some(notes) = &mut todo.notes else {return Err(AppError::NoteNotFoundError(id, index))};
    if index < notes.len() {
        notes.remove(index);
        Ok(())
    } else {
        Err(AppError::NoteNotFoundError(id, index))
    }
}
