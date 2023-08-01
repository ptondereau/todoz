use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TodoItem {
    body: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

impl TodoItem {
    pub fn new(body: String) -> Self {
        Self {
            body,
            completed: false,
            created_at: Utc::now(),
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
    }
}
