use std::fs;

use note::person::Person;
use note::storage::{load_tasks, save_tasks};
use note::task::Task;

#[test]
fn save_task_creates_json_file() {
    let path = "persistence/test_create_tasks.json";

    let tasks = vec![Task::new(
        1,
        Some(Person::new("Alice".to_string())),
        "Learn Rust testing".to_string(),
    )];

    save_tasks(path, tasks).unwrap();

    let contents = fs::read_to_string(path).unwrap();
    assert!(contents.contains("Learn Rust testing"));
    assert!(contents.contains("Alice"));
}

#[test]
fn load_tasks_json_file() {
    let path = "persistence/test_load_tasks.json";

    let tasks = vec![
        Task::new(
            1,
            Some(Person::new("Alice".to_string())),
            "Learn Rust testing".to_string(),
        ),
        Task::new(2, None, "Write integration tests".to_string()),
    ];

    save_tasks(path, tasks).unwrap();
    let loaded_tasks = load_tasks(path).unwrap();

    assert_eq!(loaded_tasks.len(), 2);
    assert_eq!(loaded_tasks[0].id, 1);
    assert_eq!(loaded_tasks[0].assignee.as_ref().unwrap().name, "Alice");
    assert_eq!(loaded_tasks[0].description, "Learn Rust testing");
    assert_eq!(loaded_tasks[0].completed, false);

    assert_eq!(loaded_tasks[1].id, 2);
    assert!(loaded_tasks[1].assignee.is_none());
    assert_eq!(loaded_tasks[1].description, "Write integration tests");
    assert_eq!(loaded_tasks[1].completed, false);
}
