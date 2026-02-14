use super::{DataType, DataValue, ModelId};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::collections::HashMap;

    #[test]
    fn test_data_type_display() {
        assert_eq!(format!("{}", DataType::Number), "Number");
        assert_eq!(format!("{}", DataType::Text), "Text");
        assert_eq!(format!("{}", DataType::Boolean), "Boolean");
        assert_eq!(format!("{}", DataType::Path), "Path");
        assert_eq!(format!("{}", DataType::Model), "Model");
        assert_eq!(format!("{}", DataType::List(Box::new(DataType::Number))), "List(Number)");
        assert_eq!(format!("{}", DataType::Dict("key".to_string(), Box::new(DataType::Text))), "Dict(key, Text)");
        assert_eq!(format!("{}", DataType::Stream(Box::new(DataType::Boolean))), "Stream(Boolean)");
    }

    #[test]
    fn test_data_value_data_type() {
        assert_eq!(DataValue::Number(42.0).data_type(), DataType::Number);
        assert_eq!(DataValue::Text("hello").data_type(), DataType::Text);
        assert_eq!(DataValue::Boolean(true).data_type(), DataType::Boolean);
        assert_eq!(DataValue::Path(PathBuf::from("/test")).data_type(), DataType::Path);
        assert_eq!(DataValue::Model(ModelId("test-model".to_string())).data_type(), DataType::Model);

        // 测试列表
        let list = DataValue::List(vec![DataValue::Number(1.0), DataValue::Number(2.0)]);
        assert_eq!(list.data_type(), DataType::List(Box::new(DataType::Number)));

        // 测试空列表
        let empty_list = DataValue::List(vec![]);
        assert_eq!(empty_list.data_type(), DataType::List(Box::new(DataType::Number)));

        // 测试字典
        let mut dict = HashMap::new();
        dict.insert("key1".to_string(), DataValue::Text("value1"));
        let dict_value = DataValue::Dict(dict);
        assert_eq!(dict_value.data_type(), DataType::Dict("key".to_string(), Box::new(DataType::Number)));
    }

    #[test]
    fn test_data_value_as_number() {
        assert_eq!(DataValue::Number(42.0).as_number(), Some(42.0));
        assert_eq!(DataValue::Boolean(true).as_number(), Some(1.0));
        assert_eq!(DataValue::Boolean(false).as_number(), Some(0.0));
        assert_eq!(DataValue::Text("123").as_number(), Some(123.0));
        assert_eq!(DataValue::Text("abc").as_number(), None);
        assert_eq!(DataValue::Path(PathBuf::from("/test")).as_number(), None);
    }

    #[test]
    fn test_data_value_as_text() {
        assert_eq!(DataValue::Text("hello").as_text(), Some("hello"));
        assert_eq!(DataValue::Number(42.0).as_text(), None);
        assert_eq!(DataValue::Boolean(true).as_text(), None);
        assert_eq!(DataValue::Path(PathBuf::from("/test")).as_text(), None);
    }

    #[test]
    fn test_data_value_convert_to() {
        // Number to other types
        let num_value = DataValue::Number(42.0);
        assert_eq!(num_value.convert_to(DataType::Number).unwrap(), DataValue::Number(42.0));
        assert_eq!(num_value.convert_to(DataType::Text).unwrap(), DataValue::Text("42"));
        assert_eq!(num_value.convert_to(DataType::Boolean).unwrap(), DataValue::Boolean(true));

        // Text to other types
        let text_value = DataValue::Text("123".to_string());
        assert_eq!(text_value.convert_to(DataType::Text).unwrap(), DataValue::Text("123".to_string()));
        assert_eq!(text_value.convert_to(DataType::Number).unwrap(), DataValue::Number(123.0));
        assert!(text_value.convert_to(DataType::Boolean).is_err()); // "123" can't be converted to boolean

        let text_bool_value = DataValue::Text("true".to_string());
        assert_eq!(text_bool_value.convert_to(DataType::Boolean).unwrap(), DataValue::Boolean(true));

        // Boolean to other types
        let bool_value = DataValue::Boolean(true);
        assert_eq!(bool_value.convert_to(DataType::Boolean).unwrap(), DataValue::Boolean(true));
        assert_eq!(bool_value.convert_to(DataType::Number).unwrap(), DataValue::Number(1.0));
        assert_eq!(bool_value.convert_to(DataType::Text).unwrap(), DataValue::Text("true".to_string()));

        // Path to other types
        let path_value = DataValue::Path(PathBuf::from("/test"));
        assert_eq!(path_value.convert_to(DataType::Path).unwrap(), DataValue::Path(PathBuf::from("/test")));
        assert_eq!(path_value.convert_to(DataType::Text).unwrap(), DataValue::Text("/test".to_string()));
    }
}
