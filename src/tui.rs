use crate::task::Task;

pub fn print_task_details(task: &Task) {
    println!("ID: {}", task.id);
    println!("Assignee: {}", task.assignee_name());
    println!("Description: {}", task.description);
    println!("Status: {}", task.status_text());
    println!("---------------------------------");
}

pub fn print_completed_tasks(tasks: &[Task]) {
    println!("Completed tasks:");
    println!("---------------------------------");
    for task in tasks {
        if task.completed {
            print_task_details(task);
        }
    }
}

pub fn print_incomplete_tasks(tasks: &[Task]) {
    println!("Incomplete tasks:");
    println!("---------------------------------");
    for task in tasks {
        if !task.completed {
            print_task_details(task);
        }
    }
}

pub fn print_task_summary(tasks: Vec<Task>) {
    let completed = &tasks.iter().filter(|task| task.completed).count();
    let incomplete = &tasks.iter().filter(|task| !task.completed).count();
    let total = &tasks.len();

    println!("Task summary:");
    println!("Total: {}", total);
    println!("Completed: {}", completed);
    println!("Incomplete: {}", incomplete);
}
