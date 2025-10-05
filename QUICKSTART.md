# 🚀 Guia Rápido de Uso

## Executar o Task Runner

### 1. Listar todas as tasks disponíveis

```bash
cargo run
# ou
cargo run -- --list
```

**Saída esperada:**
```
🎯 Task Runner em Rust

📋 Tasks configuradas:

  ✅ git-tag - Cria uma tag no repositório Git
  ❌ build - Compila o projeto

💡 Tasks disponíveis no sistema:
  • git-tag
```

### 2. Executar uma task específica

```bash
cargo run -- git-tag
```

**Saída esperada:**
```
🚀 Executando task: git-tag
📝 Cria uma tag no repositório Git e opcionalmente faz push

🏷️  Criando tag Git: v1.0.0
✅ Tag 'v1.0.0' criada com sucesso!
📤 Enviando tag para o remoto...
✅ Tag enviada para o remoto!

✨ Task 'git-tag' concluída com sucesso!
```

### 3. Executar todas as tasks habilitadas

```bash
cargo run -- --all
```

### 4. Testar erro de task não encontrada

```bash
cargo run -- task-inexistente
```

**Saída esperada:**
```
❌ Erro: Task 'task-inexistente' não encontrada. Tasks disponíveis: git-tag
```

## Compilar para Produção

Para gerar um binário otimizado:

```bash
cargo build --release
```

O executável estará em: `target/release/runner`

Uso do binário:

```bash
./target/release/runner git-tag
```

## Configuração Personalizada

Edite `tasks.toml` para personalizar suas tasks:

```toml
[[tasks]]
name = "git-tag"
description = "Cria uma tag no repositório Git"
enabled = true

[tasks.params]
tag_name = "v2.0.0"           # Mude para sua versão
message = "My release"         # Sua mensagem
push = false                   # true para fazer push automático
```

## Exemplos de Uso

### Criar uma tag local (sem push)

```toml
[tasks.params]
tag_name = "v1.0.0"
message = "First release"
push = false
```

```bash
cargo run -- git-tag
```

### Criar uma tag e fazer push

```toml
[tasks.params]
tag_name = "v1.0.1"
message = "Bug fixes"
push = true
```

```bash
cargo run -- git-tag
```

### Criar uma tag leve (sem mensagem)

```toml
[tasks.params]
tag_name = "v1.0.2"
message = ""
push = false
```

```bash
cargo run -- git-tag
```

## Troubleshooting

### Erro: "Task não encontrada"

- Verifique se a task está configurada em `tasks.toml`
- Verifique se a task está implementada e registrada

### Erro: "Task está desabilitada"

- Mude `enabled = false` para `enabled = true` em `tasks.toml`

### Erro ao criar tag Git

- Certifique-se de estar em um repositório Git
- Verifique se a tag já não existe: `git tag -l`
- Para deletar uma tag: `git tag -d nome-da-tag`

## Próximos Passos

1. ✅ Explore o código em `src/tasks/git_tag.rs`
2. ✅ Leia `TASK_IDEAS.md` para ver exemplos de outras tasks
3. ✅ Implemente sua própria task seguindo o padrão
4. ✅ Consulte o `README.md` para documentação completa
