import json
import sys
import traceback
from typing import Dict, Any

class MicroFlowRuntime:
    def __init__(self):
        self.globals = {}
        
    def execute(self, code: str, inputs: Dict[str, Any]) -> Dict[str, Any]:
        """执行代码并返回结果"""
        # 将 inputs 注入 globals
        self.globals.update(inputs)
        
        # 执行代码
        try:
            exec(code, self.globals)
        except Exception as e:
            raise Exception(f"执行错误: {str(e)}")
        
        # 捕获输出变量
        outputs = {}
        for key, value in self.globals.items():
            # 只返回以 'output_' 开头的变量
            if key.startswith('output_'):
                outputs[key] = value
        
        return outputs
    
    def handle_request(self, request_json: str) -> str:
        """处理 JSON-RPC 请求"""
        try:
            req = json.loads(request_json)
            
            if req['method'] == 'execute_python':
                code = req['params']['code']
                inputs = req['params'].get('inputs', {})
                
                result = self.execute(code, inputs)
                
                return json.dumps({
                    "jsonrpc": "2.0",
                    "result": result,
                    "id": req['id']
                })
            else:
                return json.dumps({
                    "jsonrpc": "2.0",
                    "error": {"code": -32601, "message": f"方法不存在: {req['method']}"},
                    "id": req['id']
                })
        except json.JSONDecodeError as e:
            return json.dumps({
                "jsonrpc": "2.0",
                "error": {"code": -32700, "message": f"JSON 解析错误: {str(e)}"},
                "id": None
            })
        except Exception as e:
            return json.dumps({
                "jsonrpc": "2.0",
                "error": {"code": -32603, "message": str(e), "data": traceback.format_exc()},
                "id": req.get('id')
            })

if __name__ == "__main__":
    runtime = MicroFlowRuntime()
    # 从 stdin 读取，stdout 写入（与 Rust 通信）
    while True:
        try:
            line = sys.stdin.readline()
            if not line:
                break
            response = runtime.handle_request(line)
            print(response, flush=True)
        except KeyboardInterrupt:
            break
        except Exception as e:
            # 避免进程崩溃
            error_response = json.dumps({
                "jsonrpc": "2.0",
                "error": {"code": -32603, "message": f"运行时错误: {str(e)}"},
                "id": None
            })
            print(error_response, flush=True)
