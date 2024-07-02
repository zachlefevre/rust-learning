use std::time::Duration;

use tokio::sync::oneshot;

#[tokio::main]
async fn main2() {
    tokio::select! {
        _ = tokio::time::sleep(std::time::Duration::from_secs(3)) =>
            println!("outta time"),
        _ = tokio::spawn(async {
            println!("inside the future");
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }) =>             println!("gotta go fast"),
    };
}


#[tokio::main]
async fn main3() {
    let (transmit1, receive1) = oneshot::channel();
    let (transmit2, receive2) = oneshot::channel();
    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        transmit1.send(1);
    });
    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        transmit2.send(2);
    });
    let result = tokio::select! {
        result = receive1 => result,
        result = receive2 => result
    };
    dbg!(result);
}

#[tokio::main]
async fn main4() {
    let futbad = async {
        Err::<i32, i32>(88)
    };
    let futgood = async {
        Ok::<i32, i32>(88)
    };

    tokio::select! {
        Ok(n) = futbad => println!("this will never happen {}", n),
        Ok(n) = futgood => println!("good {}", n)
    };
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = async {}, if {println!("checking"); true} => 2,
        _ = async {}, if {println!("checking2"); false} => 2
    };
}
