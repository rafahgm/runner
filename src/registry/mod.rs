mod errors;
use crate::{registry::errors::TaskError, task::{Task, TaskFactory}};
use std::collections::HashMap;

/// Registro central de todas as tasks disponíveis
pub struct TaskRegistry {
    tasks: HashMap<String, TaskFactory>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    /// Registra uma nova task
    pub fn register<F>(&mut self, name: &str, factory: F)
    where
        F: Fn() -> Box<dyn Task> + 'static,
    {
        self.tasks.insert(name.to_string(), Box::new(factory));
    }

    /// Obtém uma task pelo nome
    pub fn get(&self, name: &str) -> Result<Box<dyn Task>, TaskError> {
        self.tasks
            .get(name)
            .map(|factory| factory())
            .ok_or_else(|| {
                let available = self.list_tasks().join(", ");
                TaskError::TaskNotFound(name.to_string(), available)
            })
    }

    /// Lista todas as tasks disponíveis
    pub fn list_tasks(&self) -> Vec<String> {
        let mut tasks: Vec<String> = self.tasks.keys().cloned().collect();
        tasks.sort();
        tasks
    }
}

impl Default for TaskRegistry {
    fn default() -> Self {
        Self::new()
    }
}
