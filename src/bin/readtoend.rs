use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() {
    let mut file = File::open("/home/zlef/.bashrc").await.unwrap();
    let mut v = [0; 20];
    dbg!(file.read(&mut v).await.unwrap());
    dbg!(v);
}
