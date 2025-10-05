pub mod git_tag;

use crate::task::Task;

/// Estrutura que representa uma task registrável
pub struct TaskDescriptor {
    pub name: &'static str,
    pub factory: fn() -> Box<dyn Task>,
}

/// Retorna todas as tasks disponíveis no sistema
pub fn available_tasks() -> Vec<TaskDescriptor> {
    vec![
        TaskDescriptor {
            name: "git-tag",
            factory: || Box::new(git_tag::GitTagTask::new()),
        }
    ]
}
