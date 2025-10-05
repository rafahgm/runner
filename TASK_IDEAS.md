# Exemplos de Tasks para Implementar

Este arquivo contém ideias e exemplos de tasks que você pode implementar no seu task runner.

## 📦 Build & Deploy

### cargo-build
Compila o projeto com diferentes perfis (debug, release).

```toml
[[tasks]]
name = "cargo-build"
enabled = true
[tasks.params]
profile = "release"
features = ["feature1", "feature2"]
```

### docker-build
Constrói uma imagem Docker do projeto.

```toml
[[tasks]]
name = "docker-build"
enabled = true
[tasks.params]
image_name = "my-app"
tag = "latest"
dockerfile = "Dockerfile"
```

### deploy
Faz deploy da aplicação.

```toml
[[tasks]]
name = "deploy"
enabled = true
[tasks.params]
environment = "production"
server = "user@host"
path = "/var/www/app"
```

## 🧪 Testes

### test
Executa os testes do projeto.

```toml
[[tasks]]
name = "test"
enabled = true
[tasks.params]
all_features = true
nocapture = false
```

### coverage
Gera relatório de cobertura de código.

```toml
[[tasks]]
name = "coverage"
enabled = true
[tasks.params]
format = "html"
output_dir = "target/coverage"
```

## 📝 Documentação

### docs
Gera a documentação do projeto.

```toml
[[tasks]]
name = "docs"
enabled = true
[tasks.params]
open = true
no_deps = false
```

### changelog
Gera ou atualiza o CHANGELOG.md.

```toml
[[tasks]]
name = "changelog"
enabled = true
[tasks.params]
from_tag = "v1.0.0"
to_tag = "HEAD"
```

## 🔍 Qualidade de Código

### lint
Executa clippy para análise de código.

```toml
[[tasks]]
name = "lint"
enabled = true
[tasks.params]
fix = false
warnings_as_errors = true
```

### format
Formata o código com rustfmt.

```toml
[[tasks]]
name = "format"
enabled = true
[tasks.params]
check = false
```

### audit
Verifica vulnerabilidades nas dependências.

```toml
[[tasks]]
name = "audit"
enabled = true
[tasks.params]
deny_warnings = true
```

## 🔄 Git

### git-commit
Faz commit com mensagem formatada.

```toml
[[tasks]]
name = "git-commit"
enabled = true
[tasks.params]
type = "feat"
scope = "api"
message = "add new endpoint"
```

### git-push
Faz push com verificações.

```toml
[[tasks]]
name = "git-push"
enabled = true
[tasks.params]
branch = "main"
force = false
run_tests = true
```

## 🗄️ Database

### migrate
Executa migrações de banco de dados.

```toml
[[tasks]]
name = "migrate"
enabled = true
[tasks.params]
direction = "up"
steps = 1
```

### seed
Popula o banco de dados com dados de teste.

```toml
[[tasks]]
name = "seed"
enabled = true
[tasks.params]
environment = "development"
```

## 🧹 Limpeza

### clean
Remove artefatos de build.

```toml
[[tasks]]
name = "clean"
enabled = true
[tasks.params]
target = true
docs = false
```

## 📊 Monitoring

### benchmark
Executa benchmarks de performance.

```toml
[[tasks]]
name = "benchmark"
enabled = true
[tasks.params]
save_baseline = true
```

## 🔐 Segurança

### security-scan
Escaneia o código em busca de problemas de segurança.

```toml
[[tasks]]
name = "security-scan"
enabled = true
[tasks.params]
tools = ["cargo-audit", "cargo-deny"]
```

## 🚀 Release

### release
Processo completo de release.

```toml
[[tasks]]
name = "release"
enabled = true
[tasks.params]
version = "1.0.0"
steps = ["test", "build", "tag", "push", "publish"]
```

## 💡 Como Implementar

Para cada task acima, siga o padrão:

1. Crie um arquivo em `src/tasks/<nome_task>.rs`
2. Implemente o trait `Task`
3. Registre no `TaskRegistry`
4. Configure em `tasks.toml`

Exemplo de estrutura:

```rust
use crate::config::TaskConfig;
use crate::task::Task;
use anyhow::Result;

pub struct MinhaTask;

impl Task for MinhaTask {
    fn name(&self) -> &str {
        "minha-task"
    }
    
    fn description(&self) -> &str {
        "Descrição breve"
    }
    
    fn execute(&self, config: &TaskConfig) -> Result<()> {
        // Implementação
        Ok(())
    }
}
```
