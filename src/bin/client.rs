use bytes::Bytes;
use mini_redis::{client, Result};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        response: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        response: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, response } => {
                    let res = client.get(&key).await;
                    response.send(res).unwrap();
                }

                Command::Set { key, val, response } => {
                    let res = client.set(&key, val).await;

                    response.send(res).unwrap();
                }
            }
        }
    });

    let get_task = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Get {
            key: "foo".to_string(),
            response: resp_tx,
        };

        tx.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!("{:?}", res);
    });

    let send_task = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            response: resp_tx,
        };

        tx2.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("{:?}", res);
    });
    send_task.await.unwrap();
    get_task.await.unwrap();

    manager.await.unwrap();
}
