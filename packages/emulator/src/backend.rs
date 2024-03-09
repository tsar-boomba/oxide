use std::sync::atomic::{AtomicBool, Ordering};

use input::ButtonEvent;
use ipc::SOCKET_PATH;
use tokio::sync::{mpsc, oneshot};

use crate::{ipc::server, MAIN_THREAD, PARK_MAIN};

pub enum BackendMessage {}

pub fn start() -> (mpsc::Sender<BackendMessage>, mpsc::Receiver<ButtonEvent>) {
    let (send, mut recv) = mpsc::channel::<BackendMessage>(64);
    let (send_input_recv, recv_input_recv) = oneshot::channel();

    let server_send = send.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async move {
            let (input_recv, task) = input::input_task().await.unwrap();
            send_input_recv.send(input_recv).unwrap();

            // Spawn input task
            tokio::spawn(task);

            tokio::fs::remove_file(SOCKET_PATH).await.ok();

            // Spawn ipc server
            tokio::spawn(server(server_send));

            while let Some(message) = recv.recv().await {
                // Handle audio n stuff
            }
        });
    });

    (send, recv_input_recv.blocking_recv().unwrap())
}

static MAIN_PARKED: AtomicBool = AtomicBool::new(false);

pub fn main_parked() -> bool {
    MAIN_PARKED.load(Ordering::Relaxed)
}

pub async fn park_main() {
    if !main_parked() {
        PARK_MAIN.get().unwrap().send(()).await.unwrap();
        MAIN_PARKED.store(true, Ordering::Relaxed);
    }
}

pub fn unpark_main() {
    if main_parked() {
        MAIN_THREAD.get().unwrap().unpark();
    }
}
