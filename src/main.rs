#[tokio::main]
async fn main() {
    println!("Cycling Trainer made using Rust!");

    connect().await;
}

async fn connect() {
    println!("Connecting to trainer...");
}
