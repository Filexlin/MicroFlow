use super::{MainState, RunningSubState, StateMachineContext, StateError, ErrorInfo, NodeError, RecoveryAction};
use crate::types::DataValue;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    // 创建可恢复的错误信息
    fn create_recoverable_error() -> ErrorInfo {
        ErrorInfo {
            error: NodeError::new("Test error".to_string(), 123, Some("Test details".to_string())),
            recoverable: true,
            retry_count: 0,
            suggested_action: RecoveryAction::Retry { max_attempts: 3, backoff: Duration::from_secs(1) },
        }
    }

    // 创建不可恢复的错误信息
    fn create_unrecoverable_error() -> ErrorInfo {
        ErrorInfo {
            error: NodeError::new("Fatal error".to_string(), 456, Some("Fatal details".to_string())),
            recoverable: false,
            retry_count: 3,
            suggested_action: RecoveryAction::ImmediateFail,
        }
    }

    #[test]
    fn test_state_transition_legitimate_paths() {
        // 测试Idle -> Pending
        let mut state = MainState::Idle;
        assert!(state.transition(MainState::Pending).is_ok());
        assert_eq!(state, MainState::Pending);

        // 测试Pending -> Running
        let mut state = MainState::Pending;
        assert!(state.transition(MainState::Running(RunningSubState::Normal)).is_ok());
        assert_eq!(state, MainState::Running(RunningSubState::Normal));

        // 测试Pending -> Error
        let mut state = MainState::Pending;
        let error_info = create_recoverable_error();
        assert!(state.transition(MainState::Error(error_info.clone())).is_ok());
        match state {
            MainState::Error(info) => assert_eq!(info.error.message, "Test error"),
            _ => panic!("Expected Error state"),
        }

        // 测试Pending -> Cancelled
        let mut state = MainState::Pending;
        assert!(state.transition(MainState::Cancelled).is_ok());
        assert_eq!(state, MainState::Cancelled);

        // 测试Running -> Completed
        let mut state = MainState::Running(RunningSubState::Normal);
        assert!(state.transition(MainState::Completed).is_ok());
        assert_eq!(state, MainState::Completed);

        // 测试Running -> Error
        let mut state = MainState::Running(RunningSubState::Normal);
        let error_info = create_recoverable_error();
        assert!(state.transition(MainState::Error(error_info.clone())).is_ok());
        match state {
            MainState::Error(info) => assert_eq!(info.error.message, "Test error"),
            _ => panic!("Expected Error state"),
        }

        // 测试Running -> Cancelled
        let mut state = MainState::Running(RunningSubState::Normal);
        assert!(state.transition(MainState::Cancelled).is_ok());
        assert_eq!(state, MainState::Cancelled);

        // 测试Error -> Running（可恢复）
        let mut state = MainState::Error(create_recoverable_error());
        assert!(state.transition(MainState::Running(RunningSubState::Normal)).is_ok());
        assert_eq!(state, MainState::Running(RunningSubState::Normal));
    }

    #[test]
    fn test_state_transition_invalid_paths() {
        // 测试Idle -> Running（非法）
        let mut state = MainState::Idle;
        assert!(state.transition(MainState::Running(RunningSubState::Normal)).is_err());

        // 测试Idle -> Completed（非法）
        let mut state = MainState::Idle;
        assert!(state.transition(MainState::Completed).is_err());

        // 测试Completed -> Any（非法）
        let mut state = MainState::Completed;
        assert!(state.transition(MainState::Running(RunningSubState::Normal)).is_err());
        assert!(state.transition(MainState::Idle).is_err());

        // 测试Cancelled -> Any（非法）
        let mut state = MainState::Cancelled;
        assert!(state.transition(MainState::Running(RunningSubState::Normal)).is_err());
        assert!(state.transition(MainState::Idle).is_err());

        // 测试Error -> Any（不可恢复）
        let mut state = MainState::Error(create_unrecoverable_error());
        assert!(state.transition(MainState::Running(RunningSubState::Normal)).is_err());
        assert!(state.transition(MainState::Idle).is_err());
    }

    #[test]
    fn test_running_substate_transitions() {
        // 测试Running内部子状态转换
        let mut state = MainState::Running(RunningSubState::Normal);

        // Normal -> AwaitingInput
        assert!(state.transition(MainState::Running(RunningSubState::AwaitingInput)).is_ok());
        assert_eq!(state, MainState::Running(RunningSubState::AwaitingInput));

        // AwaitingInput -> Iterating
        let mut state = MainState::Running(RunningSubState::AwaitingInput);
        assert!(state.transition(MainState::Running(RunningSubState::Iterating { current: 0, total: Some(10) })).is_ok());
        match state {
            MainState::Running(RunningSubState::Iterating { current, total }) => {
                assert_eq!(current, 0);
                assert_eq!(total, Some(10));
            }
            _ => panic!("Expected Running::Iterating"),
        }

        // Iterating -> Branching
        let mut state = MainState::Running(RunningSubState::Iterating { current: 0, total: Some(10) });
        assert!(state.transition(MainState::Running(RunningSubState::Branching { condition: true, selected_branch: "branch1".to_string() })).is_ok());
        match state {
            MainState::Running(RunningSubState::Branching { condition, selected_branch }) => {
                assert_eq!(condition, true);
                assert_eq!(selected_branch, "branch1".to_string());
            }
            _ => panic!("Expected Running::Branching"),
        }

        // Branching -> Concurrent
        let mut state = MainState::Running(RunningSubState::Branching { condition: true, selected_branch: "branch1".to_string() });
        assert!(state.transition(MainState::Running(RunningSubState::Concurrent { active_tasks: 2 })).is_ok());
        match state {
            MainState::Running(RunningSubState::Concurrent { active_tasks }) => {
                assert_eq!(active_tasks, 2);
            }
            _ => panic!("Expected Running::Concurrent"),
        }
    }

    #[test]
    fn test_error_recovery() {
        // 测试可恢复错误转换到Running
        let mut state = MainState::Error(create_recoverable_error());
        assert!(state.transition(MainState::Running(RunningSubState::Normal)).is_ok());
        assert_eq!(state, MainState::Running(RunningSubState::Normal));

        // 测试不可恢复错误无法转换到Running
        let mut state = MainState::Error(create_unrecoverable_error());
        assert!(state.transition(MainState::Running(RunningSubState::Normal)).is_err());
    }

    #[test]
    fn test_state_machine_context() {
        // 创建状态机上下文
        let mut context = StateMachineContext::new(MainState::Idle);
        assert_eq!(context.current_state, MainState::Idle);
        assert!(context.previous_states.is_empty());
        assert!(context.transition_count.is_empty());

        // 执行第一次转换：Idle -> Pending
        assert!(context.transition(MainState::Pending).is_ok());
        assert_eq!(context.current_state, MainState::Pending);
        assert_eq!(context.previous_states.len(), 1);
        assert_eq!(context.previous_states[0], MainState::Idle);

        // 执行第二次转换：Pending -> Running
        assert!(context.transition(MainState::Running(RunningSubState::Normal)).is_ok());
        assert_eq!(context.current_state, MainState::Running(RunningSubState::Normal));
        assert_eq!(context.previous_states.len(), 2);
        assert_eq!(context.previous_states[1], MainState::Pending);

        // 验证转换计数
        let from_idle_to_pending = (MainState::Idle, MainState::Pending);
        let from_pending_to_running = (MainState::Pending, MainState::Running(RunningSubState::Normal));
        assert_eq!(context.transition_count.get(&from_idle_to_pending), Some(&1));
        assert_eq!(context.transition_count.get(&from_pending_to_running), Some(&1));
    }

    #[test]
    fn test_state_as_str() {
        // 测试主状态的字符串表示
        assert_eq!(MainState::Idle.as_str(), "Idle");
        assert_eq!(MainState::Pending.as_str(), "Pending");
        assert_eq!(MainState::Completed.as_str(), "Completed");
        assert_eq!(MainState::Error(create_recoverable_error()).as_str(), "Error");
        assert_eq!(MainState::Cancelled.as_str(), "Cancelled");

        // 测试Running子状态的字符串表示
        assert_eq!(MainState::Running(RunningSubState::Normal).as_str(), "Running::Normal");
        assert_eq!(MainState::Running(RunningSubState::AwaitingInput).as_str(), "Running::AwaitingInput");
        assert_eq!(MainState::Running(RunningSubState::Iterating { current: 0, total: Some(10) }).as_str(), "Running::Iterating");
        assert_eq!(MainState::Running(RunningSubState::Branching { condition: true, selected_branch: "branch1".to_string() }).as_str(), "Running::Branching");
        assert_eq!(MainState::Running(RunningSubState::Concurrent { active_tasks: 2 }).as_str(), "Running::Concurrent");
    }

    #[test]
    fn test_state_transition_errors() {
        // 测试非法转换错误
        let mut state = MainState::Idle;
        let result = state.transition(MainState::Completed);
        assert!(result.is_err());
        match result.unwrap_err() {
            StateError::InvalidTransition(msg) => {
                assert!(msg.contains("Cannot transition from Idle to Completed"));
            }
            _ => panic!("Expected InvalidTransition error"),
        }

        // 测试Completed状态无法转换
        let mut state = MainState::Completed;
        let result = state.transition(MainState::Running(RunningSubState::Normal));
        assert!(result.is_err());
        match result.unwrap_err() {
            StateError::InvalidTransition(msg) => {
                assert!(msg.contains("Cannot transition from Completed"));
            }
            _ => panic!("Expected InvalidTransition error"),
        }
    }
}
