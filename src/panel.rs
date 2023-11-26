use rocket::{get, post, State};
use rocket::fs::{NamedFile, relative};
use tokio::sync::mpsc::Sender;

#[get("/")]
pub async fn index() -> NamedFile {
    NamedFile::open(relative!("panel/index.html"))
        .await
        .expect("No index")
}

#[post("/send_message")]
pub async fn send_message(tx: &State<Sender<String>>) {
    tx.send("Button pressed".to_string())
        .await
        .expect("Failed to send message");
}

