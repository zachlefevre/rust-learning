use std::{time::{Instant, Duration}, future::Future, task::Poll};

struct Delay {
    instant: Instant
}

impl Future for Delay {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.instant <= Instant::now() {
            Poll::Ready(())
        } else {
            println!("not done yet");
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}


#[tokio::main]
async fn main() {
    let delay = Delay {instant: Instant::now() + Duration::from_secs(5)};
    delay.await;
    println!("done")
}
