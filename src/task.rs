use crate::person::Person;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub assignee: Option<Person>,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: usize, assignee: Option<Person>, description: String) -> Self {
        Self {
            id,
            assignee,
            description,
            completed: false,
        }
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }

    pub fn status_text(&self) -> &str {
        if self.completed { "done" } else { "not done" }
    }

    pub fn assignee_name(&self) -> &str {
        match &self.assignee {
            Some(person) => &person.name,
            None => "None",
        }
    }
}
