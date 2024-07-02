use std::time::Duration;

use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut js = JoinSet::new();

    let futures = (0..100).rev().map(|n| async move {
        println!("submitting {n}");
        tokio::time::sleep(Duration::from_secs(n)).await;
        n
    }).collect::<Vec<_>>();

    for future in futures {
        js.spawn(future);
    };


    while let Some(Ok(item)) = js.join_next().await {
        dbg!(item);
    };
}
