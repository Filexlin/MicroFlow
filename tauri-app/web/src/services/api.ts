import { invoke } from '@tauri-apps/api/tauri';
import React, { useState } from 'react';

export class MicroFlowError extends Error {
  constructor(public code: string, message: string, public originalError?: unknown) {
    super(message);
    this.name = 'MicroFlowError';
  }
  getUserMessage(): string {
    const map: Record<string, string> = {
      'INVOKE_ERROR': '命令执行失败',
      'MODEL_NOT_FOUND': '模型未找到',
      'VRAM_INSUFFICIENT': '显存不足',
      'UNKNOWN': '未知错误',
    };
    return map[this.code] || this.message;
  }
}

export async function invokeCommand<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw new MicroFlowError('INVOKE_ERROR', `命令 ${command} 失败: ${error}`, error);
  }
}

export class ErrorBoundary extends React.Component<
  { children: React.ReactNode; fallback?: (error: MicroFlowError, reset: () => void) => React.ReactNode },
  { error: MicroFlowError | null }
> {
  constructor(props: any) {
    super(props);
    this.state = { error: null };
  }
  static getDerivedStateFromError(error: Error) {
    return { error: error instanceof MicroFlowError ? error : new MicroFlowError('UNKNOWN', error.message) };
  }
  handleReset = () => this.setState({ error: null });
  render() {
    if (this.state.error) {
      return this.props.fallback ? this.props.fallback(this.state.error, this.handleReset) : (
        <div style={{ padding: 20, border: '2px solid red', background: '#ffebee' }}>
          <h3>⚠️ 出错了</h3>
          <p>代码: {this.state.error.code}</p>
          <p>消息: {this.state.error.getUserMessage()}</p>
          <button onClick={this.handleReset}>重试</button>
        </div>
      );
    }
    return this.props.children;
  }
}

export function withErrorBoundary<P extends object>(Component: React.ComponentType<P>): React.FC<P> {
  return (props) => <ErrorBoundary><Component {...props} /></ErrorBoundary>;
}

export function useAsyncCommand<T>(commandFn: () => Promise<T>) {
  const [data, setData] = useState<T | null>(null);
  const [error, setError] = useState<MicroFlowError | null>(null);
  const [loading, setLoading] = useState(false);
  const execute = async () => {
    try {
      setLoading(true);
      setError(null);
      setData(await commandFn());
    } catch (err) {
      setError(err instanceof MicroFlowError ? err : new MicroFlowError('UNKNOWN', String(err)));
    } finally {
      setLoading(false);
    }
  };
  const reset = () => { setData(null); setError(null); setLoading(false); };
  return { data, error, loading, execute, reset };
}
