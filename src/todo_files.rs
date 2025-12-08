use xdir::{config};
use std::fs::{create_dir, read_dir, File};
use std::path::PathBuf;
use std::process::Command;
use std::error::Error;
use std::os::unix::fs::symlink;

fn get_confdir() -> Result<PathBuf, Box<dyn Error>> {
    let confdir = config().map(|path| path.join("tort_todo"))
                          .unwrap_or_default();
    if confdir.exists() {
        return Ok(confdir);
    }
    create_dir(&confdir)?;
    let _out = Command::new("git")
        .args(["-C", confdir.to_str().unwrap(), "init"])
        .output()?;
    Ok(confdir)
}

fn setup_todolist() -> Result<(), Box<dyn Error>> {
    todo!();
}

fn init_todo(name: &str) -> Result<(), Box<dyn Error>> {
    let confdir = get_confdir()?;

    let created_dir = confdir.join(format!("{}.json", name));
    File::create(&created_dir)?;
    let active = confdir.join("active_todos.json");
    if !active.exists() {
        symlink(created_dir, active)?;
    }
    Ok(())
}

fn set_active(name: &str) -> Result<(), Box<dyn Error>> {
    let confdir = get_confdir()?;

    let selected_dir = confdir.join(format!("{}.json", name));
    let active = confdir.join("active_todos.json");
    if !active.exists() {
        symlink(selected_dir, active)?;
    }
    Ok(())
}

fn list_todos() -> Result<(), Box<dyn Error>>{
    let confdir = get_confdir()?;
    let contents = read_dir(confdir)?;

    for file in contents {
        let name = file?.file_name();
        println!("{}", name.to_str().unwrap_or(""));
    }
    Ok(())
}