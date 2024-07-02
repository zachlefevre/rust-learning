use tokio::*;

async fn action(i: Option<i32>) -> Option<String>{
    println!("in action {:?}", i);
    match i {
        None => None,
        Some(v) => Some("foo".into())
    }
}

#[tokio::main]
async fn main() {
    let (send, mut receive) = tokio::sync::mpsc::channel::<i32>(10);

    let mut running = false;

    let operation = action(None);
    tokio::pin!(operation);

    let _ = send.send(3).await;
    let _ = send.send(3).await;
    let _ = send.send(3).await;
    let _ = send.send(2).await;

    let v = loop {
        select! {
        res = &mut operation, if !running => {
            println!("first branch");
            running = true;
            if let Some(res) = res {
                break res
            }
        },
        Some(recv) = receive.recv() => {
            println!("second branch");
            if recv % 2 == 0 {
                operation.set(action(Some(recv)));
                running = false
            }
        }
    }};
    dbg!(v);

}

