use std::time::Duration;

#[tokio::main]
async fn main() {
    let a = async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        tokio::time::sleep(Duration::from_secs(5)).await;
        22};

    let b = async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        tokio::time::sleep(Duration::from_secs(5)).await;
        String::from("foo")
    };

    let c = async {
            String::from("foo")
    };

//    dbg!(tokio::join!(a, b, c));
    dbg!(tokio::join!(tokio::spawn(a), tokio::spawn(b), tokio::spawn(c)));

}
