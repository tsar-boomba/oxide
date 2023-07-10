use http::{Method, Request, Response};
use hyper::Body;
use hyperlocal::{UnixClientExt, Uri};
use once_cell::sync::Lazy;

use crate::{functions::Function, SOCKET_PATH};

static CLIENT: Lazy<hyper::Client<hyperlocal::UnixConnector>> = Lazy::new(|| hyper::Client::unix());

pub async fn call<F: Function>(args: F::ReqBody) -> Result<Response<Body>, hyper::Error> {
    let req = Request::builder()
        .uri(Uri::new(SOCKET_PATH, F::path()))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(hyper::body::Body::from(serde_json::to_vec(&args).unwrap()))
        .unwrap();
    CLIENT.request(req).await
}
