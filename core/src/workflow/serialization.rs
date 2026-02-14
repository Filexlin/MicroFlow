use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeData {
    pub id: String,
    pub r#type: String,
    pub position: Position,
    pub data: serde_json::Value,
    pub config: Option<serde_json::Value>,
    pub default_inputs: Option<serde_json::Value>,
    pub input_types: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdgeData {
    pub id: String,
    pub source: String,
    pub target: String,
    pub animated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowData {
    pub version: String,
    pub nodes: Vec<NodeData>,
    pub edges: Vec<EdgeData>,
}

impl WorkflowData {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    pub fn from_json(s: &str) -> Result<Self, String> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }

    pub fn get_node(&self, node_id: &str) -> Option<&NodeData> {
        self.nodes.iter().find(|node| node.id == node_id)
    }
}
