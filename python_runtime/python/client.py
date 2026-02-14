#!/usr/bin/env python3
import json, socket, sys

class Client:
    def __init__(self, addr="127.0.0.1:9944"):
        self.addr = addr
    
    def execute(self, node_id, code, inputs):
        req = {
            "jsonrpc": "2.0",
            "method": "ExecutePython",
            "params": {"node_id": node_id, "code": code, "inputs": inputs},
            "id": 1
        }
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.connect(self.addr.split(":"))
        sock.send(json.dumps(req).encode())
        resp = sock.recv(4096).decode()
        sock.close()
        return json.loads(resp)

if __name__ == "__main__":
    c = Client()
    print(c.execute("test", "print('hello')", {}))