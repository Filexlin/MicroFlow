import { create } from 'zustand';
import { Node, Edge } from '@xyflow/react';
import { invoke } from '@tauri-apps/api/core';

interface WorkflowState {
  nodes: Node[];
  edges: Edge[];
  selectedNode: Node | null;
  setNodes: (nodes: Node[]) => void;
  setEdges: (edges: Edge[]) => void;
  setSelectedNode: (node: Node | null) => void;
  updateNodeData: (nodeId: string, data: any) => void;
  validateWorkflow: () => Promise<boolean>;
  saveWorkflow: () => Promise<string>;
  loadWorkflow: (json: string) => Promise<void>;
}

export const useWorkflowStore = create<WorkflowState>((set, get) => ({
  nodes: [
    { id: '1', type: 'input', position: { x: 100, y: 100 }, data: { label: '文本输入' } },
    { id: '2', type: 'llm', position: { x: 300, y: 100 }, data: { label: 'LLM模型', modelPath: '', temperature: 0.7 } },
    { id: '3', type: 'output', position: { x: 500, y: 100 }, data: { label: '输出结果' } }
  ],
  edges: [],
  selectedNode: null,
  setNodes: (nodes) => set({ nodes }),
  setEdges: (edges) => set({ edges }),
  setSelectedNode: (node) => set({ selectedNode: node }),
  updateNodeData: (nodeId, data) => set((state) => ({
    nodes: state.nodes.map((n) =>
      n.id === nodeId ? { ...n, data: { ...n.data, ...data } } : n
    )
  })),
  validateWorkflow: async () => {
    try {
      const { edges } = get();
      const edgePairs = edges.map(e => [e.source, e.target]);
      await invoke('detect_cycles', { edges: edgePairs });
      return true;
    } catch (error) {
      alert(`验证失败: ${error}`);
      return false;
    }
  },
  saveWorkflow: async () => {
    try {
      const { nodes, edges } = get();
      const result = await invoke('save_workflow', { nodes, edges });
      return result as string;
    } catch (error) {
      alert(`保存失败: ${error}`);
      throw error;
    }
  },
  loadWorkflow: async (json) => {
    try {
      const data = await invoke('load_workflow', { json });
      const workflowData = data as any;
      set({ nodes: workflowData.nodes, edges: workflowData.edges });
    } catch (error) {
      alert(`加载失败: ${error}`);
      throw error;
    }
  }
}));

