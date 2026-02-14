import { ReactFlow, Background, Controls, MiniMap, addEdge, Handle, Position } from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import { useWorkflowStore } from '../store/workflowStore';

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
  const { nodes, edges, setNodes, setEdges, setSelectedNode } = useWorkflowStore();

  return (
    <div style={{width:'100%',height:'100vh'}}>
      <ReactFlow 
        nodes={nodes} 
        edges={edges} 
        onNodesChange={setNodes} 
        onEdgesChange={setEdges} 
        onConnect={(c)=>setEdges((eds)=>addEdge({...c,animated:true},eds))} 
        onNodeClick={(_, node) => setSelectedNode(node)}
        onPaneClick={() => setSelectedNode(null)}
        nodeTypes={nodeTypes} 
        fitView
      >
        <Controls /><MiniMap /><Background />
      </ReactFlow>
    </div>
  );
}
