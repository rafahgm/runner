use crate::config::TaskConfig;
use crate::task::Task;

pub struct GitTagTask;

impl GitTagTask {
    pub fn new() -> Self {
        Self
    }

    fn get_param_string(config: &TaskConfig, key: &str, default: &str) -> String {
        config
            .params
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or(default)
            .to_string()
    }

    fn get_param_bool(config: &TaskConfig, key: &str, default: bool) -> bool {
        config
            .params
            .get(key)
            .and_then(|v| v.as_bool())
            .unwrap_or(default)
    }
}

impl Task for GitTagTask {
    fn name(&self) -> &str {
        "git-tag"
    }

    fn description(&self) -> &str {
        "Cria uma tag no repositório Git e opcionalmente faz push"
    }

    fn execute(&self, config: &TaskConfig) -> Result<(), crate::errors::AppError> {
        println!("executando git-tag");
        Ok(())
    }
}
