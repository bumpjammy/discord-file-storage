mod bot;

#[tokio::main]
async fn main() {
    bot::create_bot().await;
}
