import { Handle, Position } from '@xyflow/react';
export default function LLMNode({ data }: any) {
  return (
    <div style={{ padding: 10, border: '1px solid #777', borderRadius: 5, background: '#e3f2fd' }}>
      <div>LLM推理</div>
      <div style={{ fontSize: 12, color: '#666' }}>{data.modelId}</div>
      <Handle type="target" position={Position.Left} />
      <Handle type="source" position={Position.Right} />
    </div>
  );
}