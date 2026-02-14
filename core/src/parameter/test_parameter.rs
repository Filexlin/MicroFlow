use super::{ParameterInstance, ExecutionContext, ParamMode, ValueSource};
use crate::types::{DataValue, DataType, Error as MicroFlowError};
use std::collections::HashMap;
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;

    // 创建测试执行上下文
    fn create_test_context() -> ExecutionContext {
        let mut port_values = HashMap::new();
        port_values.insert("input1".to_string(), DataValue::Number(42.0));
        port_values.insert("input2".to_string(), DataValue::Text("test".to_string()));
        port_values.insert("input3".to_string(), DataValue::Boolean(true));

        let mut global_vars = HashMap::new();
        global_vars.insert("pi".to_string(), DataValue::Number(3.14159));

        ExecutionContext {
            node_id: "test-node".to_string(),
            port_values,
            global_vars,
            execution_time: Instant::now(),
        }
    }

    #[test]
    fn test_parameter_instance_creation() {
        // 测试创建参数实例
        let param = ParameterInstance::new(
            "test-param".to_string(),
            DataValue::Number(100.0)
        );

        assert_eq!(param.def_id, "test-param".to_string());
        match param.mode {
            ParamMode::Constant(value) => assert_eq!(value, DataValue::Number(100.0)),
            _ => panic!("Expected Constant mode"),
        }
        assert_eq!(param.ui_state.visible, true);
        assert_eq!(param.ui_state.enabled, true);
        assert_eq!(param.ui_state.value_source, ValueSource::Default);
        assert!(param.ui_state.error_message.is_none());
    }

    #[test]
    fn test_constant_mode_resolution() {
        // 测试常量模式解析
        let param = ParameterInstance::new(
            "test-param".to_string(),
            DataValue::Number(100.0)
        );

        let ctx = create_test_context();
        let result = param.resolve(&ctx);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DataValue::Number(100.0));
    }

    #[test]
    fn test_connection_mode_resolution() {
        // 测试连接模式解析
        let mut param = ParameterInstance::new(
            "test-param".to_string(),
            DataValue::Number(100.0)
        );

        // 切换到连接模式
        param.switch_to_connection("input1".to_string());

        let ctx = create_test_context();
        let result = param.resolve(&ctx);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DataValue::Number(42.0));
    }

    #[test]
    fn test_connection_mode_with_cached_value() {
        // 测试连接模式使用缓存值
        let mut param = ParameterInstance::new(
            "test-param".to_string(),
            DataValue::Number(100.0)
        );

        // 切换到连接模式并设置缓存值
        param.switch_mode(ParamMode::Connection {
            port_id: "non-existent".to_string(),
            cached_value: Some(DataValue::Number(99.9)),
        });

        let ctx = create_test_context();
        let result = param.resolve(&ctx);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DataValue::Number(99.9));
    }

    #[test]
    fn test_connection_mode_no_value() {
        // 测试连接模式无值情况
        let mut param = ParameterInstance::new(
            "test-param".to_string(),
            DataValue::Number(100.0)
        );

        // 切换到连接模式，不设置缓存值
        param.switch_to_connection("non-existent".to_string());

        let ctx = create_test_context();
        let result = param.resolve(&ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_mode_switching() {
        // 测试模式切换
        let mut param = ParameterInstance::new(
            "test-param".to_string(),
            DataValue::Number(100.0)
        );

        // 检查初始模式
        match param.mode {
            ParamMode::Constant(_) => assert_eq!(param.ui_state.value_source, ValueSource::Default),
            _ => panic!("Expected Constant mode"),
        }

        // 切换到连接模式
        param.switch_to_connection("input1".to_string());
        match param.mode {
            ParamMode::Connection { port_id, .. } => {
                assert_eq!(port_id, "input1".to_string());
                assert_eq!(param.ui_state.value_source, ValueSource::Connection);
            }
            _ => panic!("Expected Connection mode"),
        }

        // 切换回常量模式
        param.switch_to_constant(DataValue::Text("switched back".to_string()));
        match param.mode {
            ParamMode::Constant(value) => {
                assert_eq!(value, DataValue::Text("switched back".to_string()));
                assert_eq!(param.ui_state.value_source, ValueSource::UserInput);
            }
            _ => panic!("Expected Constant mode"),
        }
    }

    #[test]
    fn test_update_cached_value() {
        // 测试更新缓存值
        let mut param = ParameterInstance::new(
            "test-param".to_string(),
            DataValue::Number(100.0)
        );

        // 切换到连接模式
        param.switch_to_connection("input1".to_string());

        // 更新缓存值
        param.update_cached_value(DataValue::Number(200.0));

        let ctx = create_test_context();
        // 测试是否使用了新的缓存值
        let result = param.resolve(&ctx);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DataValue::Number(42.0)); // 应该从上下文获取，不是缓存
    }

    #[test]
    fn test_get_type() {
        // 测试获取参数类型
        let mut param = ParameterInstance::new(
            "test-param".to_string(),
            DataValue::Number(100.0)
        );

        // 测试常量模式的类型
        assert_eq!(param.get_type(), DataType::Number);

        // 测试连接模式的类型（无缓存值）
        param.switch_to_connection("input1".to_string());
        assert_eq!(param.get_type(), DataType::Number); // 默认为Number

        // 测试连接模式的类型（有缓存值）
        param.update_cached_value(DataValue::Text("cached"));
        assert_eq!(param.get_type(), DataType::Text);
    }

    #[test]
    fn test_type_compatibility() {
        // 测试类型兼容性
        let mut param = ParameterInstance::new(
            "test-param".to_string(),
            DataValue::Number(100.0)
        );

        // 相同类型
        assert!(param.is_type_compatible(&DataType::Number));

        // 兼容类型
        assert!(param.is_type_compatible(&DataType::Text));
        assert!(param.is_type_compatible(&DataType::Boolean));

        // 不兼容类型
        assert!(!param.is_type_compatible(&DataType::Path));
        assert!(!param.is_type_compatible(&DataType::Model));

        // 测试其他类型的兼容性
        param.switch_to_constant(DataValue::Boolean(true));
        assert!(param.is_type_compatible(&DataType::Number));
        assert!(param.is_type_compatible(&DataType::Text));

        param.switch_to_constant(DataValue::Text("test"));
        assert!(param.is_type_compatible(&DataType::Number));
        assert!(param.is_type_compatible(&DataType::Boolean));
    }
}
