# 🏗️ Arquitetura do Task Runner

## Visão Geral

O task runner é composto por vários módulos que trabalham juntos para fornecer um sistema flexível e extensível.

```
┌─────────────────────────────────────────────────────────────┐
│                         main.rs                              │
│                    (Ponto de Entrada)                        │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                       runner.rs                              │
│                   (TaskRunner)                               │
│  • Carrega configuração                                      │
│  • Registra tasks                                            │
│  • Executa tasks                                             │
└───────┬─────────────────┬────────────────┬──────────────────┘
        │                 │                │
        ▼                 ▼                ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  config.rs   │  │ registry.rs  │  │   task.rs    │
│              │  │              │  │              │
│ • Config     │  │ • Registry   │  │ • Trait Task │
│ • TaskConfig │  │ • Discovery  │  │              │
└──────────────┘  └──────────────┘  └──────────────┘
                         │                │
                         │                │
                         ▼                ▼
                  ┌─────────────────────────────┐
                  │      tasks/mod.rs           │
                  │   (Módulos de Tasks)        │
                  └──────────┬──────────────────┘
                             │
                 ┌───────────┼───────────┐
                 ▼           ▼           ▼
         ┌──────────┐ ┌──────────┐ ┌──────────┐
         │ git_tag  │ │  build   │ │   ...    │
         │  .rs     │ │  .rs     │ │          │
         └──────────┘ └──────────┘ └──────────┘
```

## Fluxo de Execução

### 1. Inicialização

```rust
main()
  ├─> TaskRunner::new("tasks.toml")
  │     └─> Config::from_file()
  │           └─> Carrega e parseia tasks.toml
  │
  └─> runner.register_tasks()
        └─> Registra cada task no TaskRegistry
```

### 2. Execução de Task

```rust
runner.run_task("git-tag")
  ├─> config.find_task("git-tag")
  │     └─> Encontra TaskConfig no arquivo TOML
  │
  ├─> Verifica se enabled = true
  │
  ├─> registry.get("git-tag")
  │     └─> Retorna instância de GitTagTask
  │
  └─> task.execute(config)
        └─> Executa a lógica da task
```

### 3. Tratamento de Erros

```rust
Error Flow:
  ├─> Task não configurada em TOML
  │     └─> "Task 'X' não está configurada"
  │
  ├─> Task desabilitada (enabled = false)
  │     └─> TaskError::TaskDisabled
  │
  ├─> Task não registrada no sistema
  │     └─> TaskError::TaskNotFound
  │           └─> Mostra tasks disponíveis
  │
  └─> Erro durante execução
        └─> anyhow::Error com contexto
```

## Componentes Principais

### 1. Config (`config.rs`)

**Responsabilidade:** Gerenciar a configuração das tasks

```rust
Config {
    tasks: Vec<TaskConfig>
}

TaskConfig {
    name: String,
    description: String,
    enabled: bool,
    params: HashMap<String, toml::Value>
}
```

**Métodos:**
- `from_file(path)` - Carrega configuração do TOML
- `find_task(name)` - Busca uma task por nome

### 2. Task Trait (`task.rs`)

**Responsabilidade:** Interface que todas as tasks implementam

```rust
trait Task {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, config: &TaskConfig) -> Result<()>;
}
```

### 3. TaskRegistry (`registry.rs`)

**Responsabilidade:** Registro e descoberta de tasks em runtime

```rust
TaskRegistry {
    tasks: HashMap<String, TaskFactory>
}
```

**Métodos:**
- `register(name, factory)` - Registra uma nova task
- `get(name)` - Obtém uma task por nome
- `list_tasks()` - Lista todas as tasks disponíveis

### 4. TaskRunner (`runner.rs`)

**Responsabilidade:** Orquestração do sistema

```rust
TaskRunner {
    registry: TaskRegistry,
    config: Config
}
```

**Métodos:**
- `new(config_path)` - Inicializa o runner
- `register_tasks()` - Registra todas as tasks
- `run_task(name)` - Executa uma task específica
- `run_all()` - Executa todas as tasks habilitadas
- `list_configured_tasks()` - Lista tasks configuradas

## Padrões de Design Utilizados

### 1. **Trait Object Pattern**

```rust
Box<dyn Task>
```

Permite polimorfismo dinâmico, possibilitando armazenar diferentes tipos de tasks na mesma coleção.

### 2. **Factory Pattern**

```rust
type TaskFactory = Box<dyn Fn() -> Box<dyn Task>>;
```

Cria instâncias de tasks sob demanda, permitindo registro e descoberta dinâmica.

### 3. **Registry Pattern**

```rust
TaskRegistry
```

Mantém um registro central de todas as tasks disponíveis, permitindo descoberta em runtime.

### 4. **Configuration Object Pattern**

```rust
TaskConfig
```

Encapsula todos os parâmetros de configuração de uma task.

## Extensibilidade

### Adicionando Nova Task

1. **Criar módulo da task:**

```rust
// src/tasks/minha_task.rs
pub struct MinhaTask;

impl Task for MinhaTask {
    // implementação
}
```

2. **Registrar no mod.rs:**

```rust
// src/tasks/mod.rs
pub mod minha_task;
```

3. **Registrar no runner:**

```rust
// src/runner.rs
self.registry.register("minha-task", || {
    Box::new(crate::tasks::minha_task::MinhaTask::new())
});
```

4. **Configurar no TOML:**

```toml
[[tasks]]
name = "minha-task"
enabled = true
```

## Tratamento de Erros

O sistema usa duas bibliotecas para gerenciamento de erros:

- **`anyhow`**: Para erros genéricos com contexto
- **`thiserror`**: Para erros tipados personalizados

```rust
// Erros tipados
enum TaskError {
    TaskNotFound(String, String),
    TaskDisabled(String),
}

// Erros com contexto
task.execute(config)
    .context("Falha ao executar task")?;
```

## Considerações de Performance

- **Lazy Loading**: Tasks são criadas apenas quando necessário
- **Zero-cost Abstractions**: Uso de traits sem overhead em runtime
- **Compilação Eficiente**: Rust otimiza o código em tempo de compilação

## Segurança

- **Type Safety**: Sistema de tipos do Rust previne muitos erros
- **Ownership**: Gerenciamento de memória garantido pelo compilador
- **Error Handling**: Todos os erros são tratados explicitamente

## Futuras Melhorias

1. **Paralelização**: Executar múltiplas tasks em paralelo
2. **Dependências**: Tasks podem depender de outras
3. **Hooks**: Before/After hooks para tasks
4. **Watch Mode**: Executar tasks automaticamente em mudanças
5. **Plugins**: Sistema de plugins dinâmico
6. **UI**: Interface gráfica ou TUI
