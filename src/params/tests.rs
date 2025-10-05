use super::*;

#[test]
fn test_param_definition_builder() {
    let param = ParamDefinition::new("test", ParamType::String)
        .required()
        .description("A test parameter");

    assert_eq!(param.name, "test");
    assert!(param.required);
    assert_eq!(param.description, "A test parameter");
}

#[test]
fn test_required_param_missing() {
    let definitions = vec![ParamDefinition::new("required_param", ParamType::String).required()];

    let raw_params = HashMap::new();
    let result = TaskParams::new(&definitions, &raw_params);

    assert!(result.is_err());
}

#[test]
fn test_optional_param_with_default() {
    let definitions = vec![
        ParamDefinition::new("optional_param", ParamType::String).default_str("default_value"),
    ];

    let raw_params = HashMap::new();
    let params = TaskParams::new(&definitions, &raw_params).unwrap();

    assert_eq!(
        params.get_string("optional_param").unwrap(),
        "default_value"
    );
}

#[test]
fn test_type_validation() {
    let definitions = vec![ParamDefinition::new("bool_param", ParamType::Bool)];

    let mut raw_params = HashMap::new();
    raw_params.insert(
        "bool_param".to_string(),
        Value::String("not a bool".to_string()),
    );

    let result = TaskParams::new(&definitions, &raw_params);
    assert!(result.is_err());
}

#[test]
fn test_valid_params() {
    let definitions = vec![
        ParamDefinition::new("name", ParamType::String).required(),
        ParamDefinition::new("age", ParamType::Integer).default_int(0),
        ParamDefinition::new("active", ParamType::Bool).default_bool(true),
    ];

    let mut raw_params = HashMap::new();
    raw_params.insert("name".to_string(), Value::String("John".to_string()));
    raw_params.insert("age".to_string(), Value::Integer(30));

    let params = TaskParams::new(&definitions, &raw_params).unwrap();

    assert_eq!(params.get_string("name").unwrap(), "John");
    assert_eq!(params.get_int("age").unwrap(), 30);
    assert_eq!(params.get_bool("active").unwrap(), true);
}
