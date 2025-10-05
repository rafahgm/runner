use core::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum TaskError {
    TaskNotFound(String, String),
}

impl Display for TaskError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Self::TaskNotFound(ref task, ref available) => write!(
                f,
                "Task '{}' não encontrada. Tasks disponíveis: {}",
                task, available
            ),
        }
    }
}
