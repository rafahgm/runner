use crate::{config::TaskConfig, errors::AppError, params::{ParamDefinition, TaskParams}};

/// Trait que todas as tasks devem implementar
pub trait Task: Send + Sync {
    /// Nome único da task
    fn name(&self) -> &str;
    
    /// Descrição da task
    fn description(&self) -> &str;
    
    /// Define os parâmetros aceitos por esta task
    fn param_definitions(&self) -> Vec<ParamDefinition> {
        vec![]
    }
    
    /// Executa a task com a configuração fornecida
    fn execute(&self, config: &TaskConfig) -> Result<(), AppError> {
        // Valida e prepara os parâmetros
        let params = TaskParams::new(self.param_definitions(), config.params.clone());
        params.validate()?;

        // Chama a execução com os parâmetros validados
        self.run(&params)
    }
    
    /// Executa a task com os parâmetros validados
    fn run(&self, params: &TaskParams) -> Result<(), AppError>;
}

/// Tipo para factory de tasks
pub type TaskFactory = Box<dyn Fn() -> Box<dyn Task>>;
