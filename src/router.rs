use std::collections::HashMap;

use route_recognizer::Router as MethodRouter;

use crate::BoxHTTPHandler;

pub type Router = HashMap<String, MethodRouter<BoxHTTPHandler>>;
