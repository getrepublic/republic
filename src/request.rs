use std::net::SocketAddr;

use route_recognizer::Params;

pub type Request = hyper::Request<hyper::Body>;

pub struct RequestCtx {
    pub request: Request,
    pub params: Params,
    pub remote_addr: SocketAddr,
}
