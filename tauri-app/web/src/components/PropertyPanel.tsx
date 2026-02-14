import { useWorkflowStore } from '../store/workflowStore';

export default function PropertyPanel() {
  const { selectedNode, updateNodeData } = useWorkflowStore();
  
  if (!selectedNode) return <div style={{padding:20}}>点击节点编辑属性</div>;
  
  return (
    <div style={{padding:20,width:250,borderLeft:'1px solid #ccc',height:'100vh'}}>
      <h3>节点属性</h3>
      <p>类型: {selectedNode.type}</p>
      <label>名称:</label>
      <input
        value={selectedNode.data.label || ''}
        onChange={(e) => updateNodeData(selectedNode.id, { label: e.target.value })}
        style={{width:'100%',marginBottom:10}}
      />
      {selectedNode.type === 'llm' && (
        <>
          <label>模型路径:</label>
          <input
            value={selectedNode.data.modelPath || ''}
            onChange={(e) => updateNodeData(selectedNode.id, { modelPath: e.target.value })}
            placeholder="/path/to/model.gguf"
            style={{width:'100%'}}
          />
          <label style={{marginTop:10,display:'block'}}>温度:</label>
          <input
            type="range" min="0" max="2" step="0.1"
            value={selectedNode.data.temperature || 0.7}
            onChange={(e) => updateNodeData(selectedNode.id, { temperature: parseFloat(e.target.value) })}
          />
          <div style={{fontSize:12,color:'#666'}}>{selectedNode.data.temperature || 0.7}</div>
        </>
      )}
    </div>
  );
}
