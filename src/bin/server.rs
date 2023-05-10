use bytes::Bytes;
use mini_redis::{Command as MrCommand, Connection, Frame};
use rust_demo::RdCommand;
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);

    let manager_task = tokio::spawn(async move { manager(rx).await });
    let handle_all_task = tokio::spawn(async move { handle_all(tx).await });

    manager_task.await.unwrap();
    handle_all_task.await.unwrap();
}

async fn manager(mut rx: Receiver<RdCommand>) {
    let mut db: HashMap<String, Bytes> = HashMap::new();
    while let Some(local_cmd) = rx.recv().await {
        match local_cmd {
            RdCommand::Get { key, resp } => match db.get(&key) {
                Some(value) => {
                    resp.send(Option::Some(value.clone())).unwrap();
                }
                None => {
                    resp.send(Option::None).unwrap();
                }
            },
            RdCommand::Set { key, value, resp } => {
                db.insert(key, value);
                resp.send(()).unwrap();
            }
        };
    }
}

async fn handle_all(tx: Sender<RdCommand>) {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    while let Ok((socket, _)) = listener.accept().await {
        let connection = Connection::new(socket);
        let tx_copy = tx.clone();
        tokio::spawn(async move {
            handle_each(tx_copy, connection).await;
        });
    }
}

async fn handle_each(tx: Sender<RdCommand>, mut connection: Connection) {
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match MrCommand::from_frame(frame).unwrap() {
            MrCommand::Get(cmd) => {
                let (resp_tx, resp_rx) = oneshot::channel();
                tx.send(RdCommand::Get {
                    key: cmd.key().into(),
                    resp: resp_tx,
                })
                .await
                .unwrap();
                match resp_rx.await.unwrap() {
                    Some(bytes) => Frame::Bulk(bytes),
                    None => Frame::Null,
                }
            }
            MrCommand::Set(cmd) => {
                let (resp_tx, resp_rx) = oneshot::channel();
                tx.send(RdCommand::Set {
                    key: cmd.key().into(),
                    value: cmd.value().clone(),
                    resp: resp_tx,
                })
                .await
                .unwrap();
                resp_rx.await.unwrap();
                Frame::Simple("OK".into())
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
