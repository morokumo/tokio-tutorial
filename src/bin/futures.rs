use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello, World~");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
pub async fn main() {
    let when = Instant::now() + Duration::from_secs(3);
    let future = Delay { when };
    let out = future.await;
    assert_eq!(out, "done");
}
