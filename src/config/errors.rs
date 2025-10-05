use core::fmt;

#[derive(Debug)]
pub enum ConfigError {
  ConfigNotFoundError(String),
  ParsingError(String)
}

impl fmt::Display for ConfigError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Self::ConfigNotFoundError(ref path ) => write!(f, "Configuração não encontrada no caminho '{}'", path),
      Self::ParsingError(ref error) => write!(f, "Erro ao ler arquivo de configuraçao:\n{}", error)
    }
  }
}