use crate::protocol::{ExecuteRequest, ExecuteResponse};
use jsonrpsee::{server::Server, RpcModule};
use std::collections::HashMap;
use tokio;

pub async fn start_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::builder()
        .build(addr.parse::<std::net::SocketAddr>()?)
        .await?;
    let mut module = RpcModule::new(());

    module.register_method("ExecutePython", |params, _| {
        let _req: ExecuteRequest = params.parse()?;
        // TODO: 调用Python子进程
        Ok::<ExecuteResponse, jsonrpsee::types::ErrorObjectOwned>(ExecuteResponse {
            success: true,
            outputs: HashMap::new(),
            error: None,
        })
    })?;

    let handle = server.start(module);
    tokio::spawn(handle.stopped());
    Ok(())
}
