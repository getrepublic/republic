use std::net::SocketAddr;

use republic::{LogMiddleware, RequestCtx, Response, ResponseBuiler, Server};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();

    let mut srv = Server::new();

    srv.middleware(LogMiddleware);

    srv.get("/", |_req| async move {
        hyper::Response::builder()
            .status(hyper::StatusCode::OK)
            .body(hyper::Body::from("Welcome!"))
            .unwrap()
    });
    srv.get("/hello/:name", hello);

    srv.run(addr).await.unwrap();
}

async fn hello(ctx: RequestCtx) -> Response {
    let name = ctx.params.find("name").unwrap_or("world");

    ResponseBuiler::with_text(format!("Hello {}!", name))
}
