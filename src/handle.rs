use std::future::Future;

use crate::{RequestCtx, Response};

#[async_trait::async_trait]
pub trait HTTPHandler: Send + Sync + 'static {
    async fn handle(&self, ctx: RequestCtx) -> Response;
}

pub type BoxHTTPHandler = Box<dyn HTTPHandler>;

#[async_trait::async_trait]
impl<F: Send + Sync + 'static, Fut> HTTPHandler for F
where
    F: Fn(RequestCtx) -> Fut,
    Fut: Future<Output = Response> + Send + 'static,
{
    async fn handle(&self, ctx: RequestCtx) -> Response {
        self(ctx).await
    }
}
