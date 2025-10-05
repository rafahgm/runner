// Exemplo de implementação de uma task de build
// Para usar, descomente este código e siga as instruções no README.md

/*
use crate::config::TaskConfig;
use crate::task::Task;
use anyhow::{Context, Result};
use std::process::Command;

pub struct BuildTask;

impl BuildTask {
    pub fn new() -> Self {
        Self
    }

    fn get_param_bool(config: &TaskConfig, key: &str, default: bool) -> bool {
        config
            .params
            .get(key)
            .and_then(|v| v.as_bool())
            .unwrap_or(default)
    }
}

impl Task for BuildTask {
    fn name(&self) -> &str {
        "build"
    }

    fn description(&self) -> &str {
        "Compila o projeto Rust"
    }

    fn execute(&self, config: &TaskConfig) -> Result<()> {
        let release = Self::get_param_bool(config, "release", false);

        println!("🔨 Compilando o projeto...");

        let mut cmd = Command::new("cargo");
        cmd.arg("build");

        if release {
            cmd.arg("--release");
            println!("📦 Modo: Release (otimizado)");
        } else {
            println!("🐛 Modo: Debug");
        }

        let output = cmd
            .output()
            .context("Falha ao executar cargo build")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Erro na compilação:\n{}", error);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);

        println!("✅ Compilação concluída com sucesso!");

        Ok(())
    }
}
*/

// Para habilitar esta task:
// 1. Descomente o código acima
// 2. Adicione em src/tasks/mod.rs: pub mod build;
// 3. Registre em src/runner.rs no método register_tasks():
//    self.registry.register("build", || Box::new(crate::tasks::build::BuildTask::new()));
// 4. Habilite a task em tasks.toml mudando enabled = true
