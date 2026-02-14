import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import FlowCanvas from './components/FlowCanvas';
import PropertyPanel from './components/PropertyPanel';
import { useWorkflowStore } from './store/workflowStore';
import './App.css';

function App() {
  const [status, setStatus] = useState('');
  const { selectedNode, validateWorkflow, saveWorkflow, loadWorkflow, executeWorkflow, isExecuting, executionResult } = useWorkflowStore();

  const testBackend = async () => {
    const result = await invoke('get_system_info');
    setStatus(result as string);
    alert('Backend: ' + result);
  };

  const handleValidate = async () => {
    const isValid = await validateWorkflow();
    if (isValid) {
      alert('工作流验证通过！');
    }
  };

  const handleSave = async () => {
    try {
      await saveWorkflow();
    } catch (error) {
      console.error('保存失败:', error);
    }
  };

  const handleLoad = async () => {
    try {
      await loadWorkflow();
    } catch (error) {
      console.error('加载失败:', error);
    }
  };

  return (
    <div style={{display:'flex',height:'100vh'}}>
      <div style={{flex:1,display:'flex',flexDirection:'column'}}>
        <div style={{padding:10,borderBottom:'1px solid #ccc'}}>
          <h1>MicroFlow - Visual AI Workflow</h1>
          <div style={{marginBottom:10}}>
            <button onClick={testBackend} style={{marginRight:10}}>Test Backend</button>
            <button 
              onClick={executeWorkflow} 
              disabled={isExecuting}
              style={{marginRight:10}}
            >
              {isExecuting ? '⏳ 执行中...' : '▶ 运行工作流'}
            </button>
            <button onClick={handleValidate} style={{marginLeft:10}}>✓ 验证</button>
            <button onClick={handleSave} style={{marginLeft:10}}>💾 保存</button>
            <button onClick={handleLoad} style={{marginLeft:10}}>📂 加载</button>
            <span style={{marginLeft:10,color:'#666'}}>{status}</span>
          </div>
          {executionResult && (
            <div style={{marginTop:10, padding:10, background:'#f0f0f0', borderRadius:4}}>
              <strong>执行结果:</strong> {executionResult}
            </div>
          )}
        </div>
        <FlowCanvas />
      </div>
      {selectedNode && <PropertyPanel />}
    </div>
  );
}
export default App;
