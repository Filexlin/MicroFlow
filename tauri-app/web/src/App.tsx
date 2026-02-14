import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import FlowCanvas from './components/FlowCanvas';
import PropertyPanel from './components/PropertyPanel';
import { useWorkflowStore } from './store/workflowStore';
import './App.css';

function App() {
  const [status, setStatus] = useState('');
  const { selectedNode, validateWorkflow, saveWorkflow, loadWorkflow } = useWorkflowStore();

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
      // 1. 验证工作流
      const isValid = await validateWorkflow();
      if (!isValid) return;

      // 2. 生成JSON
      const json = await saveWorkflow();

      // 3. 触发下载
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `workflow_${new Date().toISOString().slice(0, 19).replace(/:/g, '-')}.mflow`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);

      alert('工作流保存成功！');
    } catch (error) {
      console.error('保存失败:', error);
    }
  };

  const handleLoad = () => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.mflow,application/json';
    input.onchange = async (e) => {
      const target = e.target as HTMLInputElement;
      if (target.files && target.files[0]) {
        const file = target.files[0];
        const reader = new FileReader();
        reader.onload = async (event) => {
          const json = event.target?.result as string;
          try {
            await loadWorkflow(json);
            alert('工作流加载成功！');
          } catch (error) {
            console.error('加载失败:', error);
          }
        };
        reader.readAsText(file);
      }
    };
    input.click();
  };

  return (
    <div style={{display:'flex',height:'100vh'}}>
      <div style={{flex:1,display:'flex',flexDirection:'column'}}>
        <div style={{padding:10,borderBottom:'1px solid #ccc'}}>
          <h1>MicroFlow - Visual AI Workflow</h1>
          <div style={{marginBottom:10}}>
            <button onClick={testBackend} style={{marginRight:10}}>Test Backend</button>
            <button onClick={()=>alert('Run workflow')}>▶ Run</button>
            <button onClick={handleValidate} style={{marginLeft:10}}>✓ 验证</button>
            <button onClick={handleSave} style={{marginLeft:10}}>💾 保存</button>
            <button onClick={handleLoad} style={{marginLeft:10}}>📂 加载</button>
            <span style={{marginLeft:10,color:'#666'}}>{status}</span>
          </div>
        </div>
        <FlowCanvas />
      </div>
      {selectedNode && <PropertyPanel />}
    </div>
  );
}
export default App;
