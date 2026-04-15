use crate::task::Task;
use serde_json::{to_string_pretty, from_str};
use std::fs::{read_to_string, write, create_dir_all};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

pub fn save_tasks(path: &str, tasks: Vec<Task>) -> Result<()> {
    let json = to_string_pretty(&tasks).map_err(|err| Error::new(ErrorKind::Other, err))?;

    write(path, json)?;
    Ok(())
}

pub fn load_tasks(path: &str) -> Result<Vec<Task>> {
    ensure_storage_path(path)?;

    match read_to_string(path) {
        Ok(content) => {
            let tasks = from_str(&content).map_err(|err| Error::new(ErrorKind::Other, err))?;
            Ok(tasks)
        }
        Err(err) if err.kind() == ErrorKind::NotFound => {
            write(path, "[]")?;
            Ok(Vec::new())
        }
        Err(err) => Err(err),
    }
}

fn ensure_storage_path(path: &str) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            create_dir_all(parent)?;
        }
    }

    Ok(())
}