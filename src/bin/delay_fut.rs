use std::{time, future::Future, task::Poll};

struct Delay {
    time: time::Instant
}

impl Future for Delay {
    type Output = time::Instant;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let now = time::Instant::now();
        if now >= self.time {
            Poll::Ready(now)
        } else {
            println!("{:?}", now);
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let delay = Delay { time: time::Instant::now() + time::Duration::from_secs(5) };
    dbg!(delay.await);
}
