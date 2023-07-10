use std::future::Future;

use axum::Router;
use hyper::Server;
use hyperlocal::UnixServerExt;

use crate::SOCKET_PATH;

/// Returns a future which drives the ipc server
pub fn server(router: Router) -> impl Future<Output = Result<(), hyper::Error>> {
    Server::bind_unix(SOCKET_PATH)
        .unwrap()
        .serve(router.into_make_service())
}
