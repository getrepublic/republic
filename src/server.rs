use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::service::{make_service_fn, service_fn};
use route_recognizer::{Params, Router as MethodRouter};

use crate::{
    Error, HTTPHandler, HyperRequest, Middleware, Next, RequestCtx, Response, ResponseBuiler,
    Router,
};

pub struct Server {
    router: Router,
    middlewares: Vec<Arc<dyn Middleware>>,
}

impl Server {
    pub fn new() -> Self {
        Server { router: HashMap::new(), middlewares: Vec::new() }
    }

    pub fn register(
        &mut self,
        method: impl ToString,
        path: impl AsRef<str>,
        handler: impl HTTPHandler,
    ) {
        let method = method.to_string().to_uppercase();
        self.router
            .entry(method)
            .or_insert_with(MethodRouter::new)
            .add(path.as_ref(), Box::new(handler));
    }

    register_method!(get, "GET");
    register_method!(head, "HEAD");
    register_method!(post, "POST");
    register_method!(put, "PUT");
    register_method!(delete, "DELETE");
    register_method!(connect, "CONNECT");
    register_method!(options, "OPTIONS");
    register_method!(trace, "TRACE");
    register_method!(patch, "PATCH");

    pub fn middleware(&mut self, middleware: impl Middleware) {
        self.middlewares.push(Arc::new(middleware));
    }

    pub async fn run(self, addr: SocketAddr) -> Result<(), Error> {
        let Self { router, middlewares } = self;

        let router = Arc::new(router);
        let middlewares = Arc::new(middlewares);

        let make_svc = make_service_fn(|conn: &hyper::server::conn::AddrStream| {
            let router = router.clone();
            let middlewares = middlewares.clone();
            let remote_addr = conn.remote_addr();

            async move {
                Ok::<_, Infallible>(service_fn(move |req: HyperRequest| {
                    let router = router.clone();
                    let middlewares = middlewares.clone();

                    async move {
                        let method = &req.method().as_str().to_uppercase();

                        let mut req_params = Params::new();
                        let endpoint = match router.get(method) {
                            Some(router) => match router.recognize(req.uri().path()) {
                                Ok(route_recognizer::Match { handler, params }) => {
                                    req_params = params;
                                    &**handler
                                }
                                Err(_) => &Self::handle_not_found,
                            },
                            None => &Self::handle_not_found,
                        };

                        let next = Next { endpoint, next_middleware: &middlewares };

                        let ctx = RequestCtx { request: req, params: req_params, remote_addr };

                        let resp = next.run(ctx).await;

                        Ok::<_, Infallible>(resp)
                    }
                }))
            }
        });

        let server = hyper::Server::bind(&addr).serve(make_svc);

        server.await.map_err(|e| Error::new(format!("server run error: {:?}", e)))?;

        Ok(())
    }

    async fn handle_not_found(_ctx: RequestCtx) -> Response {
        ResponseBuiler::with_status(hyper::StatusCode::NOT_FOUND)
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
