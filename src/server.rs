use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        // 處理 GET 請求
        (&Method::GET, "/") => {
            Ok(Response::new(Body::from("Hello, World!")))
        },
        // 處理 POST 請求
        (&Method::POST, "/echo") => {
            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            Ok(Response::new(Body::from(body)))
        },
        // 其他路徑返回 404
        _ => {
            let mut not_found = Response::new(Body::from("Not Found"));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() {
    // 定義伺服器綁定的地址和端口
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // 建立服務
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    // 建立伺服器
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // 啟動伺服器
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

