use bytes::Bytes;
use tokio::sync::oneshot;

type Responder<T> = oneshot::Sender<T>;

#[derive(Debug)]
pub enum RdCommand {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>,
    },
}
