mod errors;

use crate::errors::AppError;
use std::collections::HashMap;
use toml::Value;

/// Tipo de parâmetro esperado
#[derive(Debug, Clone)]
pub enum ParamType {
    String,
    Bool,
    Integer,
    Float,
    Array,
}

/// Definição de um parâmetro
#[derive(Debug, Clone)]
pub struct ParamDefinition {
    pub name: String,
    pub param_type: ParamType,
    pub required: bool,
    pub default: Option<Value>,
    pub description: String,
}

impl ParamDefinition {
    pub fn new(name: impl Into<String>, param_type: ParamType) -> Self {
        Self {
            name: name.into(),
            param_type,
            required: false,
            default: None,
            description: String::new(),
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    pub fn default_str(mut self, value: impl Into<String>) -> Self {
        self.default = Some(Value::String(value.into()));
        self
    }

    pub fn default_bool(mut self, value: bool) -> Self {
        self.default = Some(Value::Boolean(value));
        self
    }

    pub fn default_int(mut self, value: i64) -> Self {
        self.default = Some(Value::Integer(value));
        self
    }

    pub fn default_float(mut self, value: f64) -> Self {
        self.default = Some(Value::Float(value));
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
}

/// Container para parâmetros validados
#[derive(Debug, Clone)]
pub struct TaskParams {
    definitions: Vec<ParamDefinition>,
    raw_params: HashMap<String, Value>,
    params: HashMap<String, Value>,
}

impl TaskParams {
    /// Cria um novo container de parâmetros a partir das definições e valores brutos
    pub fn new(definitions: Vec<ParamDefinition>, raw_params: HashMap<String, Value>) -> Self {
        Self {
            definitions,
            raw_params,
            params: HashMap::new(),
        }
    }

    pub fn validate(&self) -> Result<(), AppError> {
        let mut params = HashMap::new();

        // Valida cada definição de parâmetro
        for def in &self.definitions {
            let value = match self.raw_params.get(&def.name) {
                Some(v) => {
                    // Valida o tipo
                    Self::validate_type(&def.name, v, &def.param_type)?;
                    v.clone()
                }
                None => {
                    // Se não foi fornecido, usa o default ou retorna erro se for required
                    if def.required {
                        return Err(AppError::Generic(format!(
                            "Parâmetro obrigatório '{}' não foi fornecido",
                            def.name
                        )));
                    }
                    def.default.clone().unwrap_or(Value::String(String::new()))
                }
            };

            params.insert(def.name.clone(), value);
        }

        Ok(())
    }

    /// Valida se o valor corresponde ao tipo esperado
    fn validate_type(name: &str, value: &Value, expected_type: &ParamType) -> Result<(), AppError> {
        let matches = match (value, expected_type) {
            (Value::String(_), ParamType::String) => true,
            (Value::Boolean(_), ParamType::Bool) => true,
            (Value::Integer(_), ParamType::Integer) => true,
            (Value::Float(_), ParamType::Float) => true,
            (Value::Array(_), ParamType::Array) => true,
            _ => false,
        };

        if !matches {
            return Err(AppError::Generic(format!(
                "Parâmetro '{}' tem tipo inválido. Esperado: {:?}, Recebido: {:?}",
                name,
                expected_type,
                Self::value_type_name(value)
            )));
        }

        Ok(())
    }

    fn value_type_name(value: &Value) -> &str {
        match value {
            Value::String(_) => "String",
            Value::Boolean(_) => "Bool",
            Value::Integer(_) => "Integer",
            Value::Float(_) => "Float",
            Value::Array(_) => "Array",
            Value::Table(_) => "Table",
            Value::Datetime(_) => "Datetime",
        }
    }

    /// Obtém um parâmetro como String
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.params
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Obtém um parâmetro como String com valor padrão
    pub fn get_string_or(&self, key: &str, default: &str) -> String {
        self.get_string(key).unwrap_or_else(|| default.to_string())
    }

    /// Obtém um parâmetro como Bool
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.params.get(key).and_then(|v| v.as_bool())
    }

    /// Obtém um parâmetro como Bool com valor padrão
    pub fn get_bool_or(&self, key: &str, default: bool) -> bool {
        self.get_bool(key).unwrap_or(default)
    }

    /// Obtém um parâmetro como Integer
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.params.get(key).and_then(|v| v.as_integer())
    }

    /// Obtém um parâmetro como Integer com valor padrão
    pub fn get_int_or(&self, key: &str, default: i64) -> i64 {
        self.get_int(key).unwrap_or(default)
    }

    /// Obtém um parâmetro como Float
    pub fn get_float(&self, key: &str) -> Option<f64> {
        self.params.get(key).and_then(|v| v.as_float())
    }

    /// Obtém um parâmetro como Float com valor padrão
    pub fn get_float_or(&self, key: &str, default: f64) -> f64 {
        self.get_float(key).unwrap_or(default)
    }

    /// Obtém um parâmetro como Array
    pub fn get_array(&self, key: &str) -> Option<&Vec<Value>> {
        self.params.get(key).and_then(|v| v.as_array())
    }

    /// Verifica se um parâmetro existe
    pub fn has(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    /// Retorna todos os parâmetros
    pub fn all(&self) -> &HashMap<String, Value> {
        &self.params
    }
}

#[cfg(test)]
mod tests;
