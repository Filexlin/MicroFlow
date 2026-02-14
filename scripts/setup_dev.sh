#!/bin/bash
# 安装sccache
cargo install sccache
# 配置环境变量
echo "export RUSTC_WRAPPER=sccache" >> ~/.bashrc
source ~/.bashrc
echo "MicroFlow dev environment ready!"