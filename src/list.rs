use colored::Colorize;
use tabled::{builder::Builder, settings::{Span, style::Style, Color, themes::Colorization}, Table};
use itertools::Itertools;
use std::iter;
use std::collections::HashMap;
use std::cmp::Ordering;
use crate::{Todo, GroupOption};
use crate::serde_date::SerdeDate;

macro_rules! bold_if {
    ($cond:expr, $val:expr) => {
        if $cond {
            $val.bold().to_string()
        } else {
            $val
        }
    }
}

fn colorize_subject(k: &str) -> String {
    k.split_whitespace().map(|word: &str| -> String {
        match word.chars().nth(0) {
            Some('+') => word.purple().to_string(),
            Some('@') => word.green().to_string(),
            _ => word.to_string()
        }
    }).join(" ")
}

fn red_if_overdue(due: &SerdeDate) -> String {
    if due.cmp(&SerdeDate::today()) == Ordering::Less {
        due.to_string()
    } else {
        due.to_string().red().to_string()
    }
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

pub fn disp_list(todos: &Vec<Todo>, grouping: Option<GroupOption>, show_notes: bool) -> Vec<(&str, Table)> {
    let mut lists: Vec<(&str, Table)> = Vec::new();
    let grouped_todo: HashMap<&str, Vec<&Todo>> = todo_grouping(todos, grouping);

    for (title, todo_group) in grouped_todo.iter() {
        let mut builder = Builder::default();
        let mut note_rows = Vec::new();
        let mut contains_unarchived_item = false;
        for item in todo_group.iter() {
            if !item.archived {
                contains_unarchived_item = true;
                let record = [
                    bold_if!(item.is_priority, item.id.to_string()), 
                    bold_if!(item.is_priority, if item.completed { "[x]" } else { "[ ]" }.to_string()),
                    bold_if!(item.is_priority, red_if_overdue(&item.due)),
                    bold_if!(item.is_priority, item.status.to_string()),
                    bold_if!(item.is_priority, colorize_subject(&item.subject)),
                ];
                builder.push_record(record);
                if show_notes {
                    if let Some(notes) = &item.notes {
                        notes.iter().enumerate().for_each(|(i, note)| {
                            note_rows.push((builder.count_records(), 2));
                            builder.push_record(["".to_string(), i.to_string(), note.to_string()]);
                        });
                    }
                }
            }
        }

        if contains_unarchived_item {
            let idcol = Color::FG_YELLOW;
            let complcol = Color::FG_BLUE;
            let duecol = Color::FG_YELLOW;
            let statuscol = Color::FG_RED;
            let subjectcol = Color::FG_BRIGHT_WHITE;

            let mut table = builder.build();
            table.with(Style::blank())
                .with(Colorization::columns([idcol, complcol, duecol, statuscol, subjectcol]));
            for row in note_rows {
                table.modify(row, Span::column(3));
            }

            lists.push((*title, table));
        }
    }
    lists
}

pub fn list(todos: &Vec<Todo>, grouping: Option<GroupOption>, show_notes: bool) -> () {
    let lists = disp_list(todos, grouping, show_notes);
    for (title, table) in lists {
        println!("{}:\n{}", title, table);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Todo, SerdeDate, SerdeDateTime};

    #[test]
    fn test_simple() {
        let serdedate = SerdeDate::try_from(Some("nov28".to_string()));

        assert!(serdedate.is_ok());

        let todo: Vec<Todo> = vec![
            Todo {
                id: 0,
                uuid: "abcde".to_string(),
                subject: "this is the subject".to_string(),
                projects: vec![],
                contexts: vec![],
                due: serdedate.unwrap(),
                completed: false,
                completed_date: SerdeDateTime::new_empty(),
                status: "waiting".to_string(),
                archived: false,
                is_priority: false,
                notes: None,
                recur: "".to_string(),
                recur_until: "".to_string(),
                prev_recur_todo_uuid: "".to_string()
            }
        ];

        let display = disp_list(&todo, None, false);
        assert_eq!(display.len(), 1);
        assert_eq!(display[0].0, "All");
        assert_eq!(display[0].1.to_string(), 
            "\u{1b}[33m \u{1b}[39m\u{1b}[33m0\u{1b}[39m\u{1b}[33m \u{1b}[39m \u{1b}[34m \u{1b}[39m\u{1b}[34m[ ]\u{1b}[39m\u{1b}[34m \u{1b}[39m \u{1b}[33m \u{1b}[39m\u{1b}[33mSat Nov 28\u{1b}[39m\u{1b}[33m \u{1b}[39m \u{1b}[31m \u{1b}[39m\u{1b}[31mwaiting\u{1b}[39m\u{1b}[31m \u{1b}[39m \u{1b}[97m \u{1b}[39m\u{1b}[97mthis is the subject\u{1b}[39m\u{1b}[97m \u{1b}[39m");
    }
}
