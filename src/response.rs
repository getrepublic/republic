pub type Response = hyper::Response<hyper::Body>;
pub struct ResponseBuiler;

impl ResponseBuiler {
    pub fn with_text(text: impl ToString) -> Response {
        hyper::Response::builder()
            .header(
                "Content-type".parse::<hyper::header::HeaderName>().unwrap(),
                "text/plain; charset=UTF-8".parse::<hyper::header::HeaderValue>().unwrap(),
            )
            .body(hyper::Body::from(text.to_string()))
            .unwrap()
    }

    pub fn with_html(text: impl ToString) -> Response {
        hyper::Response::builder()
            .header(
                "Content-type".parse::<hyper::header::HeaderName>().unwrap(),
                "text/html; charset=UTF-8".parse::<hyper::header::HeaderValue>().unwrap(),
            )
            .body(hyper::Body::from(text.to_string()))
            .unwrap()
    }

    pub fn with_status(status: hyper::StatusCode) -> Response {
        hyper::Response::builder().status(status).body(hyper::Body::empty()).unwrap()
    }
}
