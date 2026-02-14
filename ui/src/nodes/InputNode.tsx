import { Handle, Position } from '@xyflow/react';
export default function InputNode({ data }: any) {
  return (
    <div style={{ padding: 10, border: '1px solid #777', borderRadius: 5, background: '#fff' }}>
      <div>文本输入</div>
      <input value={data.text} onChange={(e) => data.text = e.target.value} style={{ width: 120 }} />
      <Handle type="source" position={Position.Right} />
    </div>
  );
}