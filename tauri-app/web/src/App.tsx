import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import FlowCanvas from './FlowCanvas';
import './App.css';

function App() {
  const [greetMsg, setGreetMsg] = useState('');

  async function greet() {
    // 调用 Rust 后端
    const msg = await invoke('greet', { name: 'User' });
    setGreetMsg(msg as string);
    alert(msg as string);
  }

  return (
    <div className="container">
      <h1>MicroFlow</h1>
      <div className="toolbar">
        <button onClick={greet}>Test Backend</button>
        <span>{greetMsg}</span>
      </div>
      <FlowCanvas />
    </div>
  );
}

export default App;
