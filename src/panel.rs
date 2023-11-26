use rocket::{get, post, State};
use tokio::sync::mpsc::Sender;
use crate::message::{Message, MessageChannel, MessageType};

#[post("/send_message")]
pub async fn send_message(channel: &State<MessageChannel>) {
    channel.1.send(
            Message(
                MessageType::SendMessage,
                Some("Button Pressed".to_string())
            )
        )
        .await
        .expect("Failed to send message");
}

#[get("/get_files")]
pub async fn get_files(channel: &State<MessageChannel>) -> String {
    channel.1.send(
        Message(
            MessageType::GetFiles,
            None
        )
    )
    .await
    .expect("Failed to send message");

    while let Some(msg) = channel.0.lock().await.recv().await {
        match msg.0 {
            MessageType::GetFiles => {
                return msg.1.expect("No files returned");
            }
            _ => {}
        }
    }

    return "".to_string();
}