use std::future::Future;

use ipc::{
    extract::Json,
    functions::{Function, SaveState, SaveStateArgs, Start, StartArgs, Stop, StopArgs},
    routing::post,
    Router, StatusCode,
};
use tokio::sync::mpsc;

use crate::{backend::{park_main, unpark_main, BackendMessage}, core::save::save, ARGS};

pub fn server(
    message_sender: mpsc::Sender<BackendMessage>,
) -> impl Future<Output = Result<(), ipc::Error>> {
    let router = Router::new()
        .route(
            SaveState::path(),
            post(
                |Json(SaveStateArgs { slot }): Json<<SaveState as Function>::ReqBody>| async move {
                    match save(slot.clone()).await {
                        Ok(_) => {
                            // All save ops went off with no problem
                            StatusCode::OK
                        }
                        Err(err) => {
                            tracing::error!(
                                "Error creating save state: slot: {:?} {:#?}, {err:?}",
                                slot,
                                ARGS
                            );
                            StatusCode::INTERNAL_SERVER_ERROR
                        }
                    }
                },
            ),
        )
        .route(
            Stop::path(),
            post(|Json(StopArgs {}): Json<<Stop as Function>::ReqBody>| async move {
                park_main().await;
            }),
        )
        .route(
            Start::path(),
            post(|Json(StartArgs {}): Json<<Start as Function>::ReqBody>| async move {
                unpark_main();
            }),
        )
        .with_state(message_sender);

    ipc::server::server(router)
}
