use colored::Colorize;
use tabled::{builder::Builder, settings::{Span, style::Style, Color, themes::Colorization}};
use itertools::Itertools;
use std::iter;
use std::collections::HashMap;
use crate::{Todo, GroupOption};

fn colorize_subject(k: &String) -> String {
    k.split_whitespace().map(|word: &str| -> String {
        match word.chars().nth(0) {
            Some('+') => word.purple().to_string(),
            Some('@') => word.green().to_string(),
            _ => word.to_string()
        }
    }).join(" ")
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

pub fn list(todos: &Vec<Todo>, grouping: Option<GroupOption>, show_notes: bool) -> () {
    let grouped_todo: HashMap<&str, Vec<&Todo>> = todo_grouping(todos, grouping);

    for (title, todo_group) in grouped_todo.iter() {
        let mut builder = Builder::default();
        let mut note_rows = Vec::new();
        for item in todo_group.iter() {
            if !item.archived {
                builder.push_record([
                    item.id.to_string(), 
                    if item.completed { "[x]" } else { "[ ]" }.to_string(),
                    item.due.to_string(),
                    item.status.to_string(),
                    colorize_subject(&item.subject)
                ]);
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
        println!("{}:\n{}", title, table);
    }
}
