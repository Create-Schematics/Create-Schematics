use std::time::Instant;

use poem::{Request, Response, Endpoint};
use tracing::info;

pub async fn middleware_log<E>(next: E, req: Request) -> poem::Result<Response> 
where
    E: Endpoint
{    
    let method = req.method().clone();
    let url = req.original_uri().clone();

    let endpoint = url.path();

    let address = req
        .remote_addr()
        .as_socket_addr()
        .map(|x| x.ip().to_string())
        .unwrap_or("<unknown>".into());
    
    let start = Instant::now();
    let response = next.get_response(req).await;
    let elapsed = start.elapsed();
    
    let status = response.status();

    info!(%address, %method, %status, %endpoint, ?elapsed, "Request");
    
    Ok(response)
}