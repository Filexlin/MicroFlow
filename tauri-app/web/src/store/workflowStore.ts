import { create } from 'zustand';
import { Node, Edge } from 'reactflow';

interface WorkflowState {
  nodes: Node[];
  edges: Edge[];
  selectedNode: Node | null;
  setNodes: (nodes: Node[]) => void;
  setEdges: (edges: Edge[]) => void;
  setSelectedNode: (node: Node | null) => void;
  updateNodeData: (nodeId: string, data: any) => void;
}

export const useWorkflowStore = create<WorkflowState>((set) => ({
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
}));
