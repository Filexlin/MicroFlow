import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import FlowCanvas from './components/FlowCanvas';
import './App.css';

function App() {
  const [status, setStatus] = useState('');

  const testBackend = async () => {
    const result = await invoke('get_system_info');
    setStatus(result as string);
    alert('Backend: ' + result);
  };

  return (
    <div style={{padding:10}}>
      <h1>MicroFlow - Visual AI Workflow</h1>
      <div style={{marginBottom:10}}>
        <button onClick={testBackend} style={{marginRight:10}}>Test Backend</button>
        <button onClick={()=>alert('Run workflow')}>▶ Run</button>
        <span style={{marginLeft:10,color:'#666'}}>{status}</span>
      </div>
      <FlowCanvas />
    </div>
  );
}
export default App;
