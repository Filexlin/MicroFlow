import { useCallback } from 'react';
import { ReactFlow, Background, Controls, addEdge, useNodesState, useEdgesState } from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import InputNode from './nodes/InputNode';
import LLMNode from './nodes/LLMNode';
import OutputNode from './nodes/OutputNode';
import { invoke } from '@tauri-apps/api/core';

const nodeTypes = { input: InputNode, llm: LLMNode, output: OutputNode };

const initialNodes = [
  { id: '1', type: 'input', position: { x: 100, y: 100 }, data: { text: '你好' } },
  { id: '2', type: 'llm', position: { x: 300, y: 100 }, data: { modelId: 'test' } },
  { id: '3', type: 'output', position: { x: 500, y: 100 }, data: {} },
];

export default function Flow() {
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const onConnect = useCallback((conn) => setEdges((eds) => addEdge(conn, eds)), [setEdges]);
  
  const onRun = async () => {
    const result = await invoke('execute_workflow', { nodes });
    console.log('结果:', result);
  };

  return (
    <div style={{ width: '100vw', height: '100vh' }}>
      <button onClick={onRun} style={{ position: 'absolute', zIndex: 10, top: 10, left: 10 }}>
        运行
      </button>
      <ReactFlow nodes={nodes} edges={edges} onNodesChange={onNodesChange} onEdgesChange={onEdgesChange} onConnect={onConnect} nodeTypes={nodeTypes} fitView>
        <Controls /><Background gap={12} size={1} />
      </ReactFlow>
    </div>
  );
}