use xdir::config;
use std::fs::{create_dir, read_dir, File, read_link, remove_file, write, remove_dir_all};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::error::Error;
use std::os::unix::fs::symlink;
use crate::AppError;

fn get_confdir() -> Result<PathBuf, Box<dyn Error>> {
    let confdir = config().map(|path| path.join("tort_todo"))
                          .unwrap_or_default();
    if confdir.exists() {
        return Ok(confdir);
    }
    Err(Box::new(AppError::NoConfigurationDirectory))
}

fn create_confdir() -> Result<PathBuf, Box<dyn Error>> {
    let confdir = config().map(|path| path.join("tort_todo"))
                          .unwrap_or_default();
    create_dir(&confdir)?;
    create_dir(&confdir.join("todolists"))?;
    let _out = Command::new("git")
        .args(["-C", confdir.to_str().unwrap(), "init"])
        .output()?;
    Ok(confdir)
}

pub fn run_git_commands(a: &Vec<String>) -> Result<String, Box<dyn Error>> {
    let confdir = config().map(|path| path.join("tort_todo"))
                          .unwrap_or_default();
    let out = Command::new("git")
        .args(["-C", confdir.to_str().unwrap()])
        .args(a)
        .stderr(Stdio::inherit())
        .output()?;
    Ok(String::from_utf8(out.stdout)?)
}

pub fn get_active_todo() -> Result<PathBuf, Box<dyn Error>> {
    get_confdir().map(|v| v.join("active_todos.json"))
}

pub fn init_todo(name: &str) -> Result<(), Box<dyn Error>> {
    let confdir = get_confdir().or(create_confdir())?;

    let created_dir = confdir.join("todolists")
                             .join(format!("{}.json", name));
    File::create(&created_dir)?;
    write(&created_dir, "[]")?;
    let active = confdir.join("active_todos.json");
    if !active.exists() {
        symlink(created_dir, active)?;
    }
    Ok(())
}

pub fn set_active(name: &str) -> Result<(), Box<dyn Error>> {
    let confdir = get_confdir().or(create_confdir())?;

    let selected_dir = confdir.join("todolists")
                              .join(format!("{}.json", name));
    let active = confdir.join("active_todos.json");
    if active.exists() {
        remove_file(&active)?;
    }
    symlink(selected_dir, active)?;
    Ok(())
}

pub fn list_todos() -> Result<(), Box<dyn Error>>{
    let confdir = get_confdir().or(create_confdir())?;
    let mut contents = read_dir(confdir.join("todolists"))?.peekable();
    if contents.peek().is_none() {
        println!("no todos yet!");
        return Ok(());
    }

    let link = read_link(confdir.join("active_todos.json")).ok();
    let active_file = match link {
        Some(f) => f.file_name()
                    .and_then(|x| x.to_str()
                                   .map(|v| v.to_string())),
        None => None
    };

    for file in contents {
        let name = file?.file_name();
        let mut name_without_extension = PathBuf::from(&name);
        name_without_extension.set_extension("");
        if name.to_str() == active_file.as_deref() {
            println!("{} (active)", name_without_extension.display()); 
        } else {
            println!("{}", name_without_extension.display());
        }
    }
    Ok(())
}

pub fn delete_todolist(name: &str) -> Result<(), Box<dyn Error>> {
    let confdir = get_confdir()?;
    remove_file(confdir.join("todolists").join(format!("{}.json", name)))?;
    let mut link = read_link(confdir.join("active_todos.json"))?;
    link.set_extension("");
    let active_file = link.file_name()
                          .and_then(|x| x.to_str());
    if active_file == Some(name) {
        remove_file(confdir.join("active_todos.json"))?;
    }
    Ok(())
}

pub fn nuke_all_todolists() -> Result<(), Box<dyn Error>> {
    let confdir = get_confdir()?;
    remove_dir_all(&confdir)?;
    println!("{} has been nuked. Kaboom.", confdir.display());
    Ok(())
}