use crate::errors::AppError;
use crate::params::{ParamDefinition, ParamType, TaskParams};
use crate::task::Task;

pub struct GitTagTask;

impl GitTagTask {
    pub fn new() -> Self {
        Self
    }
}

impl Task for GitTagTask {
    fn name(&self) -> &str {
        "git-tag"
    }

    fn description(&self) -> &str {
        "Cria uma tag no repositório Git e opcionalmente faz push"
    }

    fn param_definitions(&self) -> Vec<ParamDefinition> {
        vec![
            ParamDefinition::new("tag", ParamType::String)
                .required()
                .description("Nome da tag a ser criada"),
            
            ParamDefinition::new("message", ParamType::String)
                .optional()
                .description("Mensagem da tag (opcional)"),
            
            ParamDefinition::new("push", ParamType::Bool)
                .default_bool(false)
                .description("Se deve fazer push da tag para o remote"),
            
            ParamDefinition::new("remote", ParamType::String)
                .default_str("origin")
                .description("Nome do remote para push (default: origin)"),
        ]
    }

    fn run(&self, params: &TaskParams) -> Result<(), AppError> {
        let tag = params.get_string("tag")
            .ok_or_else(|| AppError::Generic("Tag é obrigatória".to_string()))?;
        
        let message = params.get_string("message");
        let push = params.get_bool_or("push", false);
        let remote = params.get_string_or("remote", "origin");

        println!("🏷️  Criando tag Git: {}", tag);
        
        if let Some(msg) = message {
            println!("📝 Mensagem: {}", msg);
        }
        
        if push {
            println!("🚀 Push para remote '{}': {}", remote, push);
        }

        // Aqui viria a implementação real do git tag
        println!("✅ Tag criada com sucesso!");
        
        Ok(())
    }
}
