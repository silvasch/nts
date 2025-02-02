#[tokio::main]
async fn main() {
    if let Err(e) = nts::run().await {
        eprintln!("{}", e);
    }
}
