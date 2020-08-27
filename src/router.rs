use crate::BoxHTTPHandler;
use route_recognizer::Router as MethodRouter;
use std::collections::HashMap;

pub type Router = HashMap<String, MethodRouter<BoxHTTPHandler>>;
