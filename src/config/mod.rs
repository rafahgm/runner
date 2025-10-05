mod errors;

pub use errors::ConfigError;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};
use toml::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub tasks: HashMap<String, TaskConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskConfig {
    pub enabled: bool,
    pub params: HashMap<String, toml::Value>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config, ConfigError> {
        // Resolve o caminho
        let absolute_path = fs::canonicalize(Path::new(path))
            .map_err(|_| ConfigError::ConfigNotFoundError(path.to_string()))?;

        let content = std::fs::read_to_string(&absolute_path).map_err(|_| {
            ConfigError::ConfigNotFoundError(absolute_path.to_str().unwrap_or(path).to_string())
        })?;

        let config_file: Value =
            toml::from_str(&content).map_err(|err| ConfigError::ParsingError(err.to_string()))?;

        let mut tasks = HashMap::new();

        // Mapeia as tabelas do arquivo de configuração para tasks e parametros
        if let Some(table) = config_file.as_table() {
            for (task, params) in table {
                if let Some(params_table) = params.as_table() {
                    let params_map: HashMap<String, Value> = params_table
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();

                    tasks.insert(
                        task.clone(),
                        TaskConfig {
                            enabled: true,
                            params: params_map,
                        },
                    );
                }
            }
        }

        return Ok(Config { tasks });
    }

    pub fn find_task(&self, task_name: &str) -> &TaskConfig {
        self.tasks.get(task_name).unwrap()
    }
}
