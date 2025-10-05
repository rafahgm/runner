// use crate::config::{Config, TaskConfig};
// use crate::registry::{TaskError, TaskRegistry};
use crate::{config::Config, errors::AppError, registry::TaskRegistry, task::Task};

pub struct TaskRunner {
    registry: TaskRegistry,
    config: Config,
}

impl TaskRunner {
    pub fn new(config_path: &str) -> Result<Self, AppError> {
        let config = Config::from_file(config_path)?;

        let registry = TaskRegistry::new();
        
        Ok(TaskRunner { config, registry })
    }

    /// Registra automaticamente todas as tasks disponíveis
    pub fn register_tasks(&mut self) {
        // Obtém todas as tasks disponíveis do módulo tasks
        for task_descriptor in crate::tasks::available_tasks() {
            let factory: fn() -> Box<dyn Task> = task_descriptor.factory;
            self.registry.register(task_descriptor.name, move || factory());
        }
    }

    pub fn run_task(&self, task_name: &str) {
        let task_config = self.config.find_task(task_name);
        let task = self.registry.get(task_name).unwrap();

        task.execute(task_config).unwrap();
    }

    pub fn run_all(&self) {
        let enabled_tasks: Vec<&String> = self.config.tasks.keys().collect();

        for task_name in enabled_tasks {
            self.run_task(task_name);
        }
    }

    // /// Registra todas as tasks disponíveis
    // pub fn register_tasks(&mut self) {
    //     // Registra a task git-tag
    //     self.registry.register("git-tag", || {
    //         Box::new(crate::tasks::git_tag::GitTagTask::new())
    //     });

    //     // Adicione mais tasks aqui conforme as criar
    //     // self.registry.register("build", || Box::new(crate::tasks::build::BuildTask::new()));
    // }

    // /// Lista todas as tasks configuradas
    // pub fn list_configured_tasks(&self) {
    //     println!("\n📋 Tasks configuradas:\n");

    //     for task in &self.config.tasks {
    //         let status = if task.enabled { "✅" } else { "❌" };
    //         println!("  {} {} - {}", status, task.name, task.description);
    //     }

    //     println!("\n💡 Tasks disponíveis no sistema:");
    //     for task_name in self.registry.list_tasks() {
    //         println!("  • {}", task_name);
    //     }
    //     println!();
    // }

    // /// Executa uma task específica
    // pub fn run_task(&self, task_name: &str) -> Result<()> {
    //     // Verifica se a task está configurada
    //     let task_config = self
    //         .config
    //         .find_task(task_name)
    //         .ok_or_else(|| {
    //             anyhow::anyhow!(
    //                 "Task '{}' não está configurada no arquivo tasks.toml",
    //                 task_name
    //             )
    //         })?;

    //     // Verifica se a task está habilitada
    //     if !task_config.enabled {
    //         return Err(TaskError::TaskDisabled(task_name.to_string()).into());
    //     }

    //     // Obtém a task do registro
    //     let task = self.registry.get(task_name)?;

    //     println!("\n🚀 Executando task: {}", task.name());
    //     println!("📝 {}\n", task.description());

    //     // Executa a task
    //     task.execute(task_config)?;

    //     println!("\n✨ Task '{}' concluída com sucesso!\n", task_name);

    //     Ok(())
    // }

    // /// Executa todas as tasks habilitadas
    // pub fn run_all(&self) -> Result<()> {
    //     let enabled_tasks: Vec<&TaskConfig> = self
    //         .config
    //         .tasks
    //         .iter()
    //         .filter(|t| t.enabled)
    //         .collect();

    //     if enabled_tasks.is_empty() {
    //         println!("⚠️  Nenhuma task habilitada para executar");
    //         return Ok(());
    //     }

    //     println!("\n🚀 Executando {} task(s) habilitada(s)...\n", enabled_tasks.len());

    //     for task_config in enabled_tasks {
    //         if let Err(e) = self.run_task(&task_config.name) {
    //             eprintln!("❌ Erro ao executar task '{}': {}", task_config.name, e);
    //             return Err(e);
    //         }
    //     }

    //     println!("✨ Todas as tasks foram concluídas!\n");

    //     Ok(())
    // }
}
