use input::ButtonEvent;
use ipc::SOCKET_PATH;
use tokio::sync::{mpsc, oneshot};

use crate::ipc::server;

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
