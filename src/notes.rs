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
        if notes.len() == 0 {
            todo.notes = None;
        }
        Ok(())
    } else {
        Err(AppError::NoteNotFoundError(id, index))
    }
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
    fn test_add_note() {
        let mut todos = gen_todo();

        let r = add_note(&mut todos, 0, "this is a note".to_string());

        assert!(r.is_ok());
        assert!(&todos[0].notes.is_some());
        assert_eq!(todos[0].notes.as_ref().unwrap()[0], "this is a note");
        assert_eq!(todos[0].notes.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_add_note_nonexistent_todo() {
        let mut todos = Vec::new();

        let r = add_note(&mut todos, 0, "this is a note".to_string());

        assert!(r.is_err());
        assert_eq!(r, Err(AppError::IdNotFoundError(0)));
    }

    #[test]
    fn test_edit_note() {
        let mut todos = gen_todo();
        todos[0].notes = Some(vec!["this is a note".to_string()]);

        let r = edit_note(&mut todos, 0, 0, "this is an edited note".to_string());

        assert!(r.is_ok());
        assert!(todos[0].notes.is_some());
        assert_eq!(todos[0].notes.as_ref().unwrap()[0], "this is an edited note");
        assert_eq!(todos[0].notes.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_edit_nonexistent_note() {
        let mut todos = gen_todo();
        todos[0].notes = Some(vec!["this is a note".to_string()]);

        let r = edit_note(&mut todos, 0, 1, "this is an edited note".to_string());

        assert!(r.is_err());
        assert_eq!(r, Err(AppError::NoteNotFoundError(0, 1)));
        assert!(&todos[0].notes.is_some());
        assert_eq!(todos[0].notes.as_ref().unwrap()[0], "this is a note");
        assert_eq!(todos[0].notes.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_delete_note() {
        let mut todos = gen_todo();
        todos[0].notes = Some(vec!["this is a note".to_string()]);

        let r = delete_note(&mut todos, 0, 0);

        assert!(r.is_ok());
        assert!(todos[0].notes.is_none());
    }

    #[test]
    fn test_delete_nonexistent_note() {
        let mut todos = gen_todo();
        todos[0].notes = Some(vec!["this is a note".to_string()]);

        let r = delete_note(&mut todos, 0, 1);

        assert!(r.is_err());
        assert_eq!(r, Err(AppError::NoteNotFoundError(0, 1)));
        assert!(&todos[0].notes.is_some());
        assert_eq!(todos[0].notes.as_ref().unwrap()[0], "this is a note");
        assert_eq!(todos[0].notes.as_ref().unwrap().len(), 1);
    }
}
