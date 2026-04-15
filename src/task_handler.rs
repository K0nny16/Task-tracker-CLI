use crate::person::Person;
use crate::storage::{load_tasks, save_tasks};
use crate::task::Task;
use crate::tui::{print_completed_tasks, print_incomplete_tasks, print_task_summary};
use std::io::{Error, ErrorKind, Result};

const PATH: &str = "persistence/tasks.json";

pub fn handle_add_task(description: String, person: Option<String>) -> Result<()> {
    let mut tasks = load_tasks(PATH)?;
    let next_id = find_next_id(&tasks);
    let assignee = person.map(Person::new);

    let task = Task::new(next_id, assignee, description);
    tasks.push(task);
    save_tasks(PATH, tasks)?;

    Ok(())
}

pub fn show_task_list() -> Result<()> {
    let tasks = load_tasks(PATH)?;
    print_completed_tasks(&tasks);
    print_incomplete_tasks(&tasks);
    print_task_summary(tasks);

    Ok(())
}

pub fn complete_task(id: usize) -> Result<()> {
    let mut tasks = load_tasks(PATH)?;
    validate_task_status_change(true, &mut tasks, id)?;

    save_tasks(PATH, tasks)?;
    Ok(())
}

pub fn delete_task(id: usize) -> Result<()> {
    let mut tasks = load_tasks(PATH)?;
    validate_delete_task(&mut tasks, id)?;

    save_tasks(PATH, tasks)?;
    Ok(())
}

fn validate_task_status_change(is_completed: bool, tasks: &mut [Task], id: usize) -> Result<()> {
    match tasks.iter_mut().find(|task| task.id == id) {
        Some(task) => {
            if task.completed == is_completed {
                return Err(Error::new(
                    ErrorKind::PermissionDenied,
                    format!("Task is already {}", task.status_text()),
                ));
            }
            task.toggle_completed();
            Ok(())
        }
        None => Err(Error::new(ErrorKind::NotFound, "Task id not found!")),
    }
}

fn validate_delete_task(tasks: &mut Vec<Task>, id: usize) -> Result<()> {
    match tasks.iter().position(|task| task.id == id) {
        Some(index) => {
            tasks.remove(index);
            Ok(())
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            format!("Could not find task with id: {}", id),
        )),
    }
}

fn find_next_id(tasks: &[Task]) -> usize {
    match tasks.iter().map(|task| task.id).max() {
        Some(max_id) => max_id + 1,
        None => 1,
    }
}
