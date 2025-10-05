use crate::{config::TaskConfig, errors::AppError};

/// Trait que todas as tasks devem implementar
pub trait Task: Send + Sync {
    /// Nome único da task
    fn name(&self) -> &str;
    
    /// Descrição da task
    fn description(&self) -> &str;
    
    /// Executa a task com a configuração fornecida
    fn execute(&self, config: &TaskConfig) -> Result<(), AppError>;
}

/// Tipo para factory de tasks
pub type TaskFactory = Box<dyn Fn() -> Box<dyn Task>>;
