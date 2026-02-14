use crate::types::DataValue;
use crate::workflow::context::ExecutionContext;
use crate::workflow::nodes::{LLMNode, TextInputNode, TextOutputNode};
use crate::workflow::serialization::{NodeData, WorkflowData};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct ExecutionResult {
    pub final_outputs: HashMap<String, DataValue>,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkflowError {
    #[error("未知节点类型: {0}")]
    UnknownNodeType(String),
    #[error("缺少配置")]
    MissingConfig,
    #[error("缺少输入")]
    MissingInput,
    #[error("循环依赖")]
    CycleDetected,
    #[error("节点未找到: {0}")]
    NodeNotFound(String),
}

pub struct WorkflowExecutor {
    ctx: ExecutionContext,
}

impl WorkflowExecutor {
    pub fn new(ctx: ExecutionContext) -> Self {
        Self { ctx }
    }

    pub fn run_simple_workflow(&self, input_text: &str, model_id: &str) {
        println!("开始执行工作流...");
        let input_node = TextInputNode::new(input_text);
        let prompt = input_node.execute();
        println!("输入: {}", prompt);

        let llm_node = LLMNode::new(model_id);
        let response = llm_node.execute(&prompt, &self.ctx);
        println!("推理: {}", response);

        let mut output_node = TextOutputNode::new();
        output_node.execute(&response);

        println!("工作流完成!");
    }

    pub async fn execute_workflow(
        &self,
        workflow: &WorkflowData,
    ) -> Result<ExecutionResult, WorkflowError> {
        // 1. 拓扑排序获取执行顺序
        let execution_order = self.topological_sort(workflow)?;

        let mut context = ExecutionContext::new();

        // 2. 按顺序执行节点
        for node_id in execution_order {
            let node = workflow
                .get_node(&node_id)
                .ok_or(WorkflowError::NodeNotFound(node_id.clone()))?;
            let inputs = self.collect_inputs(node, &context)?;

            // 3. 根据节点类型执行
            let outputs = match node.r#type.as_str() {
                "input" => self.execute_input_node(node, inputs)?,
                "llm" => self.execute_llm_node(node, inputs).await?,
                "output" => self.execute_output_node(node, inputs)?,
                _ => return Err(WorkflowError::UnknownNodeType(node.r#type.clone())),
            };

            // 4. 存储结果到上下文
            context.set_outputs(node_id, outputs);
        }

        Ok(ExecutionResult {
            final_outputs: context.get_final_outputs(),
        })
    }

    fn topological_sort(&self, workflow: &WorkflowData) -> Result<Vec<String>, WorkflowError> {
        // 构建邻接表和入度表
        let mut adjacency: HashMap<&String, Vec<&String>> = HashMap::new();
        let mut in_degree: HashMap<&String, usize> = HashMap::new();
        let mut all_nodes: HashSet<&String> = HashSet::new();

        // 初始化所有节点
        for node in &workflow.nodes {
            all_nodes.insert(&node.id);
            in_degree.entry(&node.id).or_insert(0);
        }

        // 构建边
        for edge in &workflow.edges {
            adjacency
                .entry(&edge.source)
                .or_default()
                .push(&edge.target);
            *in_degree.entry(&edge.target).or_insert(0) += 1;
        }

        // 拓扑排序
        let mut queue = VecDeque::new();
        for (node, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(*node);
            }
        }

        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());

            if let Some(neighbors) = adjacency.get(node) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(*neighbor);
                    }
                }
            }
        }

        // 检查是否有循环
        if result.len() != all_nodes.len() {
            return Err(WorkflowError::CycleDetected);
        }

        Ok(result)
    }

    fn collect_inputs(
        &self,
        _node: &NodeData,
        _context: &ExecutionContext,
    ) -> Result<HashMap<String, DataValue>, WorkflowError> {
        // 简化版：从上下文收集输入
        Ok(HashMap::new())
    }

    fn execute_input_node(
        &self,
        node: &NodeData,
        _inputs: HashMap<String, DataValue>,
    ) -> Result<HashMap<String, DataValue>, WorkflowError> {
        // 从节点数据中获取输入文本
        let text = node
            .data
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let mut outputs = HashMap::new();
        outputs.insert("text".to_string(), DataValue::Text(text));
        Ok(outputs)
    }

    async fn execute_llm_node(
        &self,
        _node: &NodeData,
        inputs: HashMap<String, DataValue>,
    ) -> Result<HashMap<String, DataValue>, WorkflowError> {
        // 从输入中获取文本
        let prompt = inputs
            .get("text")
            .map(|v| v.to_string())
            .unwrap_or("".to_string());

        // 简化版：生成回复
        let result = format!("AI回复: 收到 '{}'", prompt);

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), DataValue::Text(result));
        Ok(outputs)
    }

    fn execute_output_node(
        &self,
        _node: &NodeData,
        inputs: HashMap<String, DataValue>,
    ) -> Result<HashMap<String, DataValue>, WorkflowError> {
        // 从输入中获取结果
        let result = inputs
            .get("result")
            .map(|v| v.to_string())
            .unwrap_or("".to_string());
        let mut outputs = HashMap::new();
        outputs.insert("output".to_string(), DataValue::Text(result));
        Ok(outputs)
    }
}
