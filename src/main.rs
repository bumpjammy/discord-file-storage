use rocket::{launch, routes};
use rocket::fs::{FileServer, relative};
use tokio::sync::mpsc;
use crate::message::MessageChannel;

mod bot;
mod panel;
mod message;

#[launch]
async fn rocket() -> _ {
    let (bot, panel) = MessageChannel::new();

    tokio::spawn(async {
        bot::create_bot(bot).await;
    });

    rocket::build()
        .manage(panel)
        .mount("/", routes![panel::send_message, panel::get_files])
        .mount("/", FileServer::from(relative!("/panel")))
}
