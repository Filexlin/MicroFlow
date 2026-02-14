import { Handle, Position } from '@xyflow/react';
export default function OutputNode({ data }: any) {
  return (
    <div style={{ padding: 10, border: '1px solid #777', borderRadius: 5, background: '#e8f5e9' }}>
      <div>文本输出</div>
      <Handle type="target" position={Position.Left} />
    </div>
  );
}