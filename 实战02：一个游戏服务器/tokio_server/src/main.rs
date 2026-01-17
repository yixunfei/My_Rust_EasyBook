use std::sync::Arc;

use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;

mod app_state;
mod protocol;
mod redis_cache;
mod mongo_persist;
mod types;
mod protocol_helpers;

use app_state::AppState;
use types::{Request, Response};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // 初始化全局状态
    let state = AppState::initialize().await?;
    let state = Arc::new(state);

    // 监听端口
    let listener = TcpListener::bind("0.0.0.0:5000").await?;
    println!("Server listening on 0.0.0.0:5000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Accepted connection from {}", addr);

        let st = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, st).await {
                eprintln!("Connection error: {:?}", e);
            }
        });
    }
}

// 处理单个客户端连接
async fn handle_connection(mut stream: TcpStream, state: Arc<AppState>) -> Result<()> {
    loop {
        // 读取一个 frame：变长前缀长度 + 压缩 payload
        let payload = protocol::read_frame(&mut stream).await?;

        // 解析请求
        let req: Request = serde_json::from_slice(&payload)?;
        // 处理请求
        let resp = process_request(req, state.as_ref()).await;
        // 序列化返回、压缩、发送
        let resp_bytes = serde_json::to_vec(&resp)?;
        let framed = protocol::frame(&resp_bytes)?;
        stream.write_all(&framed).await?;
    }
}

// 请求分发
async fn process_request(req: Request, state: &AppState) -> Response {
    match req.op.as_str() {
        "cache.get" => {
            match state.redis.as_ref().get(&req.key).await {
                Ok(Some(v)) => Response { ok: true, data: Some(serde_json::Value::String(v)), error: None },
                Ok(None) => Response { ok: false, data: None, error: Some("MISS".into()) },
                Err(e) => Response { ok: false, data: None, error: Some(e.to_string()) },
            }
        }
        "cache.set" => {
            // 要求 value 为字符串
            let value = req.value.and_then(|v| v.as_str().map(|s| s.to_owned())).unwrap_or_default();
            let ttl = req.ttl.unwrap_or(0);
            match state.redis.as_ref().set(&req.key, value, ttl).await {
                Ok(_) => Response { ok: true, data: None, error: None },
                Err(e) => Response { ok: false, data: None, error: Some(e.to_string()) },
            }
        }
        "mongo.set" => {
            // 将 value 存入 Mongo，key 作为 _id
            match state.mongo.as_ref().set(&req.key, req.value).await {
                Ok(_) => Response { ok: true, data: None, error: None },
                Err(e) => Response { ok: false, data: None, error: Some(e.to_string()) },
            }
        }
        "mongo.get" => {
            match state.mongo.as_ref().get(&req.key).await {
                Ok(Some(v)) => Response { ok: true, data: Some(serde_json::Value::String(v)), error: None },
                Ok(None) => Response { ok: false, data: None, error: Some("NOT_FOUND".into()) },
                Err(e) => Response { ok: false, data: None, error: Some(e.to_string()) },
            }
        }
        _ => Response { ok: false, data: None, error: Some("unsupported_op".into()) },
    }
}
