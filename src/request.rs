use route_recognizer::Params;
use std::net::SocketAddr;

pub type HyperRequest = hyper::Request<hyper::Body>;

pub struct RequestCtx {
    pub request: HyperRequest,
    pub params: Params,
    pub remote_addr: SocketAddr,
}
