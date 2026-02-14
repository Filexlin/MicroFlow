import { ReactFlow, Background, Controls, MiniMap, useNodesState, useEdgesState, addEdge, Handle, Position } from '@xyflow/react';
import '@xyflow/react/dist/style.css';

// 基础节点组件
const InputNode = ({ data }: any) => (
  <div style={{padding:10,border:'1px solid #777',background:'#e3f2fd',borderRadius:5}}>
    <Handle type="source" position={Position.Right} />
    <div>📥 Input: {data.label}</div>
  </div>
);

const LLMNode = ({ data }: any) => (
  <div style={{padding:10,border:'1px solid #777',background:'#fff3e0',borderRadius:5}}>
    <Handle type="target" position={Position.Left} />
    <div>🤖 LLM: {data.label}</div>
    <Handle type="source" position={Position.Right} />
  </div>
);

const OutputNode = ({ data }: any) => (
  <div style={{padding:10,border:'1px solid #777',background:'#e8f5e9',borderRadius:5}}>
    <Handle type="target" position={Position.Left} />
    <div>📤 Output: {data.label}</div>
  </div>
);

const nodeTypes = { input: InputNode, llm: LLMNode, output: OutputNode };

export default function FlowCanvas() {
  const [nodes, setNodes, onNodesChange] = useNodesState([
    { id: '1', type: 'input', position: { x: 100, y: 100 }, data: { label: 'Text' } },
    { id: '2', type: 'llm', position: { x: 300, y: 100 }, data: { label: 'GPT' } },
    { id: '3', type: 'output', position: { x: 500, y: 100 }, data: { label: 'Result' } }
  ]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  return (
    <div style={{width:'100%',height:'80vh'}}>
      <ReactFlow nodes={nodes} edges={edges} onNodesChange={onNodesChange} onEdgesChange={onEdgesChange} onConnect={(c)=>setEdges((eds)=>addEdge({...c,animated:true},eds))} nodeTypes={nodeTypes} fitView>
        <Controls /><MiniMap /><Background />
      </ReactFlow>
    </div>
  );
}
