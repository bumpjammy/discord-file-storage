use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};

pub enum MessageType {
    SendMessage,
    GetFiles,
}

pub struct Message(pub MessageType, pub Option<String>);

pub struct MessageChannel(pub Arc<Mutex<Receiver<Message>>>, pub Sender<Message>);

impl MessageChannel {
    pub fn new() -> (MessageChannel, MessageChannel) {
        let (tx1, rx1) = mpsc::channel(100);
        let (tx2, rx2) = mpsc::channel(100);
        (MessageChannel(Arc::new(Mutex::new(rx1)), tx2), MessageChannel(Arc::new(Mutex::new(rx2)), tx1))
    }
}