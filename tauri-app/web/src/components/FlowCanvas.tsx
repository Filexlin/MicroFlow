import React, { useCallback, useMemo } from 'react';
import {
  ReactFlow, Node, Edge, Controls, Background, useNodesState, useEdgesState,
  addEdge, Connection, NodeTypes, NodeProps, Handle, Position,
} from 'reactflow';
import 'reactflow/dist/style.css';

interface NodeData {
  label: string;
}

const InputNode = React.memo<NodeProps<NodeData>>(({ data }) => (
  <div style={{ padding: 10, border: '2px solid #2196F3', background: '#e3f2fd', borderRadius: 8, minWidth: 150 }}>
    <Handle type="source" position={Position.Right} style={{ background: '#2196F3' }} />
    <div style={{ fontWeight: 600 }}>📥 输入节点</div>
    <div style={{ fontSize: 12 }}>{data.label}</div>
  </div>
));
InputNode.displayName = 'InputNode';

const LLMNode = React.memo<NodeProps<NodeData>>(({ data }) => (
  <div style={{ padding: 10, border: '2px solid #FF9800', background: '#fff3e0', borderRadius: 8, minWidth: 150 }}>
    <Handle type="target" position={Position.Left} style={{ background: '#FF9800' }} />
    <div style={{ fontWeight: 600 }}>🤖 LLM节点</div>
    <div style={{ fontSize: 12 }}>{data.label}</div>
    <Handle type="source" position={Position.Right} style={{ background: '#FF9800' }} />
  </div>
));
LLMNode.displayName = 'LLMNode';

const OutputNode = React.memo<NodeProps<NodeData>>(({ data }) => (
  <div style={{ padding: 10, border: '2px solid #4CAF50', background: '#e8f5e9', borderRadius: 8, minWidth: 150 }}>
    <Handle type="target" position={Position.Left} style={{ background: '#4CAF50' }} />
    <div style={{ fontWeight: 600 }}>📤 输出节点</div>
    <div style={{ fontSize: 12 }}>{data.label}</div>
  </div>
));
OutputNode.displayName = 'OutputNode';

const nodeTypes: NodeTypes = { input: InputNode, llm: LLMNode, output: OutputNode };

const initialNodes: Node<NodeData>[] = [
  { id: '1', type: 'input', position: { x: 50, y: 100 }, data: { label: '用户输入' } },
  { id: '2', type: 'llm', position: { x: 300, y: 100 }, data: { label: 'Qwen2.5' } },
  { id: '3', type: 'output', position: { x: 550, y: 100 }, data: { label: '显示结果' } },
];

const FlowCanvas: React.FC = () => {
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  const onConnect = useCallback(
    (connection: Connection) => setEdges((eds) => addEdge(connection, eds)),
    [setEdges]
  );

  return (
    <div style={{ width: '100%', height: '80vh' }}>
      <ReactFlow
        nodes={nodes} edges={edges}
        onNodesChange={onNodesChange} onEdgesChange={onEdgesChange} onConnect={onConnect}
        nodeTypes={nodeTypes} fitView
        onlyRenderVisibleElements={true}
      >
        <Background /><Controls />
      </ReactFlow>
    </div>
  );
};

export default React.memo(FlowCanvas);
