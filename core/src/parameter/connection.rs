pub type NodeId = String;

use std::collections::HashMap;

pub type PortId = String;
#[derive(Debug, Clone)]
pub struct Connection { pub from_node: NodeId, pub from_port: PortId, pub to_node: NodeId, pub to_port: PortId }

pub struct ConnectionGraph {
    connections: Vec<Connection>,
    incoming: HashMap<(NodeId, PortId), (NodeId, PortId)>,
}

impl ConnectionGraph {
    pub fn new() -> Self { Self { connections: Vec::new(), incoming: HashMap::new() } }
    pub fn connect(&mut self, conn: Connection) {
        let to_key = (conn.to_node.clone(), conn.to_port.clone());
        let from_key = (conn.from_node.clone(), conn.from_port.clone());
        self.incoming.insert(to_key, from_key);
        self.connections.push(conn);
    }
    pub fn get_source(&self, node: &NodeId, port: &PortId) -> Option<&(NodeId, PortId)> {
        self.incoming.get(&(node.clone(), port.clone()))
    }
}