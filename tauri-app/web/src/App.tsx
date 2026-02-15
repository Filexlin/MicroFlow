import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import FlowCanvas from './components/FlowCanvas';
import PropertyPanel from './components/PropertyPanel';
import { useWorkflowStore } from './store/workflowStore';
import './App.css';

function App() {
  const [status, setStatus] = useState('');
  const { selectedNode } = useWorkflowStore();

  const testBackend = async () => {
    const result = await invoke('get_system_info');
    setStatus(result as string);
    alert('Backend: ' + result);
  };

  return (
    <div style={{display:'flex',height:'100vh'}}>
      <div style={{flex:1,display:'flex',flexDirection:'column'}}>
        <div style={{padding:10,borderBottom:'1px solid #ccc'}}>
          <h1>MicroFlow - Visual AI Workflow</h1>
          <div style={{marginBottom:10}}>
            <button onClick={testBackend} style={{marginRight:10}}>Test Backend</button>
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
