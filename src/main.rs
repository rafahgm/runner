// mod config;
// mod registry;
// mod runner;
// mod task;
// mod tasks;
mod config;
mod errors;
mod params;
mod runner;
mod registry;
mod task;
mod tasks;

use cliparser::{App, Command, Flag, FlagType, flag::FlagValue, ui::ColoredUI};
use runner::TaskRunner;
use std::{env, process};

use crate::errors::AppError;

fn main() {
    let app = App::new("runner", env!("CARGO_PKG_VERSION")).add_command(
        Command::new("run")
            .add_flag(
                Flag::new("config", FlagType::String)
                    .default_value(FlagValue::String("tasks.toml".to_string())),
            )
            .show_help_on_empty(false),
    );

    match app.run_from_env() {
        Ok(parsed) => {
            let config_path = parsed
                .get_flag("config")
                .and_then(|v| v.as_string())
                .unwrap_or("tasks.toml");

            let mut runner: TaskRunner = TaskRunner::new(config_path).unwrap_or_else(|err| {
                eprintln!("❌ Erro: {}", err);
                process::exit(1);
            });
            
            // Registra as tasks disponíveis
            runner.register_tasks();
            runner.run_all();

        }
        Err(_) => process::exit(1),
    }
}

// fn main() {
//     if let Err(e) = run() {
//         eprintln!("❌ Erro: {}", e);
//         std::process::exit(1);
//     }
// }

// fn run() -> anyhow::Result<()> {
//     let args: Vec<String> = env::args().collect();

//     // Cria o runner e registra as tasks
//     let mut runner = TaskRunner::new("tasks.toml")?;
//     runner.register_tasks();

//     // Processa argumentos
//     match args.len() {
//         1 => {
//             // Sem argumentos: lista as tasks
//             println!("\n🎯 Task Runner em Rust\n");
//             runner.list_configured_tasks();
//             println!("Uso:");
//             println!("  {} <task-name>  - Executa uma task específica", args[0]);
//             println!("  {} --all        - Executa todas as tasks habilitadas", args[0]);
//             println!("  {} --list       - Lista as tasks configuradas\n", args[0]);
//         }
//         2 => {
//             let command = &args[1];
//             match command.as_str() {
//                 "--list" => {
//                     runner.list_configured_tasks();
//                 }
//                 "--all" => {
//                     runner.run_all()?;
//                 }
//                 task_name => {
//                     runner.run_task(task_name)?;
//                 }
//             }
//         }
//         _ => {
//             eprintln!("Uso: {} [task-name|--all|--list]", args[0]);
//             std::process::exit(1);
//         }
//     }

//     Ok(())
// }
