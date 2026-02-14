use std::collections::HashMap;
use std::time::Duration;

use crate::types::{DataValue, Error as MicroFlowError};

// 边ID类型（用于Branching子状态）
pub type EdgeId = String;

// 节点错误类型（用于ErrorInfo）
#[derive(Debug, Clone)]
pub struct NodeError {
    pub message: String,
    pub code: u32,
    pub details: Option<String>,
}

impl NodeError {
    pub fn new(message: String, code: u32, details: Option<String>) -> Self {
        Self {
            message,
            code,
            details,
        }
    }
}

// 恢复动作枚举
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryAction {
    ImmediateFail,
    Retry { max_attempts: u32, backoff: Duration },
    Fallback { value: DataValue },
    Skip,
}

// 错误信息结构
#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub error: NodeError,
    pub recoverable: bool,
    pub retry_count: u32,
    pub suggested_action: RecoveryAction,
}

// 子状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum RunningSubState {
    Normal,
    AwaitingInput,
    Iterating {
        current: usize,
        total: Option<usize>,  // None表示无限循环
    },
    Branching {
        condition: bool,
        selected_branch: EdgeId,
    },
    Concurrent {
        active_tasks: usize,
    },
}

// 主状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum MainState {
    Idle,
    Pending,  // 等待依赖完成
    Running(RunningSubState),  // 嵌入子状态
    Completed,
    Error(ErrorInfo),
    Cancelled,
}

// 状态错误
#[derive(Debug, Clone, PartialEq)]
pub enum StateError {
    InvalidTransition(String),
    RecoveryFailed(String),
    StateAlreadySet(String),
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateError::InvalidTransition(msg) => write!(f, "Invalid transition: {}", msg),
            StateError::RecoveryFailed(msg) => write!(f, "Recovery failed: {}", msg),
            StateError::StateAlreadySet(msg) => write!(f, "State already set: {}", msg),
        }
    }
}

impl std::error::Error for StateError {}

// 主状态实现
impl MainState {
    /// 检查是否可以转换到目标状态
    pub fn can_transition_to(&self, new: &MainState) -> bool {
        match (self, new) {
            // Idle只能转换到Pending
            (MainState::Idle, MainState::Pending) => true,
            (MainState::Idle, _) => false,
            
            // Pending可以转换到Running、Error、Cancelled
            (MainState::Pending, MainState::Running(_)) => true,
            (MainState::Pending, MainState::Error(_)) => true,
            (MainState::Pending, MainState::Cancelled) => true,
            (MainState::Pending, _) => false,
            
            // Running可以转换到Running（子状态变化）、Completed、Error、Cancelled
            (MainState::Running(_), MainState::Running(_)) => true,
            (MainState::Running(_), MainState::Completed) => true,
            (MainState::Running(_), MainState::Error(_)) => true,
            (MainState::Running(_), MainState::Cancelled) => true,
            (MainState::Running(_), _) => false,
            
            // Completed不能转换到任何状态
            (MainState::Completed, _) => false,
            
            // Error只能转换到Running（如果可恢复）
            (MainState::Error(info), MainState::Running(_)) => info.recoverable,
            (MainState::Error(_), _) => false,
            
            // Cancelled不能转换到任何状态
            (MainState::Cancelled, _) => false,
        }
    }
    
    /// 执行状态转换，返回错误如果转换非法
    pub fn transition(&mut self, new: MainState) -> Result<(), StateError> {
        if !self.can_transition_to(&new) {
            return Err(StateError::InvalidTransition(format!(
                "Cannot transition from {} to {}",
                self.as_str(),
                new.as_str()
            )));
        }
        
        *self = new;
        Ok(())
    }
    
    /// 获取当前状态的字符串表示（用于日志）
    pub fn as_str(&self) -> &'static str {
        match self {
            MainState::Idle => "Idle",
            MainState::Pending => "Pending",
            MainState::Running(substate) => match substate {
                RunningSubState::Normal => "Running::Normal",
                RunningSubState::AwaitingInput => "Running::AwaitingInput",
                RunningSubState::Iterating { .. } => "Running::Iterating",
                RunningSubState::Branching { .. } => "Running::Branching",
                RunningSubState::Concurrent { .. } => "Running::Concurrent",
            },
            MainState::Completed => "Completed",
            MainState::Error(_) => "Error",
            MainState::Cancelled => "Cancelled",
        }
    }
}

// 状态机上下文（用于执行）
pub struct StateMachineContext {
    pub current_state: MainState,
    pub previous_states: Vec<MainState>,  // 历史记录
    pub transition_count: HashMap<(MainState, MainState), usize>,
}

impl StateMachineContext {
    pub fn new(initial_state: MainState) -> Self {
        Self {
            current_state: initial_state,
            previous_states: Vec::new(),
            transition_count: HashMap::new(),
        }
    }
    
    /// 执行状态转换并记录历史
    pub fn transition(&mut self, new_state: MainState) -> Result<(), StateError> {
        let old_state = self.current_state.clone();
        
        // 执行状态转换
        self.current_state.transition(new_state)?;
        
        // 记录历史状态
        self.previous_states.push(old_state.clone());
        
        // 更新转换计数
        let key = (old_state, self.current_state.clone());
        *self.transition_count.entry(key).or_insert(0) += 1;
        
        Ok(())
    }
    
    /// 获取当前状态
    pub fn current_state(&self) -> &MainState {
        &self.current_state
    }
    
    /// 获取历史状态
    pub fn previous_states(&self) -> &Vec<MainState> {
        &self.previous_states
    }
    
    /// 获取转换计数
    pub fn transition_count(&self) -> &HashMap<(MainState, MainState), usize> {
        &self.transition_count
    }
}
