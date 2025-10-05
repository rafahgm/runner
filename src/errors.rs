use core::fmt;

#[derive(Debug)]
pub enum AppError {
    Config(crate::config::ConfigError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Config(err) => write!(f, "Erro de configuração: {}", err),
        }
    }
}

impl std::error::Error for AppError {}

impl From<crate::config::ConfigError> for AppError {
    fn from(err: crate::config::ConfigError) -> Self {
        AppError::Config(err)
    }
}