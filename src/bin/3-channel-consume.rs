#[tokio::main]
async fn main() {
    let (t1, mut r1) = tokio::sync::mpsc::channel::<i32>(20);
    let (t2, mut r2) = tokio::sync::mpsc::channel::<i32>(20);
    let (t3, mut r3) = tokio::sync::mpsc::channel::<i32>(20);

    t1.send(1).await;
    t2.send(2).await;
    t3.send(3).await;

    loop {
        let chosen = tokio::select! {
            Some(v) = r1.recv() => v,
            Some(v) = r2.recv() => v,
            Some(v) = r3.recv() => v,
            else => break
        };
        dbg!(chosen);
    };
}
