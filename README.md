# Task Runner em Rust 🦀

Um task runner modular e extensível escrito em Rust, com configuração em TOML.

## 🎯 Características

- ✅ Configuração simples em TOML
- 🔌 Sistema de plugins modular
- 🔍 Descoberta automática de tasks em runtime
- ⚡ Performance e segurança do Rust
- 📝 Mensagens de erro claras e informativas

## 🚀 Instalação

```bash
cargo build --release
```

## 📋 Configuração

Edite o arquivo `tasks.toml` para configurar suas tasks:

```toml
[[tasks]]
name = "git-tag"
description = "Cria uma tag no repositório Git"
enabled = true

[tasks.params]
tag_name = "v1.0.0"
message = "Release version 1.0.0"
push = true
```

## 💻 Uso

### Listar tasks disponíveis

```bash
cargo run
# ou
cargo run -- --list
```

### Executar uma task específica

```bash
cargo run -- git-tag
```

### Executar todas as tasks habilitadas

```bash
cargo run -- --all
```

## 🔧 Tasks Disponíveis

### git-tag

Cria uma tag no repositório Git e opcionalmente faz push para o remoto.

**Parâmetros:**

- `tag_name` (string): Nome da tag (padrão: "v0.1.0")
- `message` (string): Mensagem da tag (padrão: "")
- `push` (bool): Se deve fazer push da tag (padrão: false)

**Exemplo de configuração:**

```toml
[[tasks]]
name = "git-tag"
description = "Cria uma tag no repositório Git"
enabled = true

[tasks.params]
tag_name = "v2.0.0"
message = "Major release with breaking changes"
push = true
```

## 🛠️ Como Adicionar Novas Tasks

1. **Crie um novo módulo** em `src/tasks/`:

```rust
// src/tasks/minha_task.rs
use crate::config::TaskConfig;
use crate::task::Task;
use anyhow::Result;

pub struct MinhaTask;

impl MinhaTask {
    pub fn new() -> Self {
        Self
    }
}

impl Task for MinhaTask {
    fn name(&self) -> &str {
        "minha-task"
    }

    fn description(&self) -> &str {
        "Descrição da minha task"
    }

    fn execute(&self, config: &TaskConfig) -> Result<()> {
        println!("Executando minha task!");
        // Sua lógica aqui
        Ok(())
    }
}
```

2. **Registre o módulo** em `src/tasks/mod.rs`:

```rust
pub mod git_tag;
pub mod minha_task;
```

3. **Registre a task** em `src/runner.rs` no método `register_tasks()`:

```rust
self.registry.register("minha-task", || {
    Box::new(crate::tasks::minha_task::MinhaTask::new())
});
```

4. **Configure a task** em `tasks.toml`:

```toml
[[tasks]]
name = "minha-task"
description = "Minha task personalizada"
enabled = true

[tasks.params]
parametro1 = "valor"
parametro2 = 123
```

## 📦 Estrutura do Projeto

```
runner/
├── src/
│   ├── main.rs          # Ponto de entrada
│   ├── config.rs        # Carregamento de configuração TOML
│   ├── task.rs          # Trait base para tasks
│   ├── registry.rs      # Registro e descoberta de tasks
│   ├── runner.rs        # Executor de tasks
│   └── tasks/           # Módulos de tasks
│       ├── mod.rs
│       └── git_tag.rs   # Task de exemplo
├── tasks.toml           # Arquivo de configuração
└── Cargo.toml
```

## 🎨 Tratamento de Erros

O sistema fornece mensagens de erro claras:

- ❌ **Task não encontrada**: Mostra quais tasks estão disponíveis
- ❌ **Task desabilitada**: Informa que a task existe mas está desabilitada
- ❌ **Erro de configuração**: Detalhes sobre problemas no arquivo TOML
- ❌ **Erro de execução**: Stack trace completo para debugging

## 📝 Licença

MIT
