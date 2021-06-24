use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task;

#[tokio::main]
async fn main() -> io::Result<()> {


    async {
        task::spawn(async {
            // ...
            println!("spawned task done! 2")
        });
        task::spawn(async {
            // ...
            println!("spawned task done! 1")
        });
    
        // Yield, allowing the newly-spawned task to execute first.
        task::yield_now().await;
        task::yield_now().await;
        task::yield_now().await;
        println!("main task done!");
    }.await;
    
    let listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });
    }
}