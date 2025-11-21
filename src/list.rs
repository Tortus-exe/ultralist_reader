use colored::Colorize;
use tabled::{Table, settings::{style::Style, Color, themes::Colorization}, Tabled};
use itertools::Itertools;
use std::iter;
use std::collections::HashMap;
use crate::{Todo, GroupOption};
use crate::serde_date::SerdeDate;

#[derive(Tabled)]
struct TodoDisplay<'a> {
    id: u64,
    #[tabled(display("display_completed"))]
    completed: bool,
    due: &'a SerdeDate,
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

pub fn list(todos: &Vec<Todo>, grouping: Option<GroupOption>) -> () {
    let grouped_todo: HashMap<&str, Vec<&Todo>> = todo_grouping(todos, grouping);

    for (title, todo_group) in grouped_todo.iter() {
        let mut todos_display = Vec::new();
        for item in todo_group.iter() {
            if !item.archived {
                todos_display.push(TodoDisplay {
                    id: item.id,
                    completed: item.completed,
                    due: &item.due,
                    status: &item.status,
                    subject: colorize_subject(&item.subject)
                });
            }
        }

        let idcol = Color::FG_YELLOW;
        let complcol = Color::FG_BLUE;
        let duecol = Color::FG_YELLOW;
        let statuscol = Color::FG_RED;
        let subjectcol = Color::FG_BRIGHT_WHITE;

        let mut table = Table::new(todos_display);
        table.with(Style::blank())
        .with(Colorization::columns([idcol, complcol, duecol, statuscol, subjectcol]));
        println!("{}:\n{}", title, table);
    }
}
