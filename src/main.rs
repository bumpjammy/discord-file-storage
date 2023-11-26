use rocket::{launch, routes};
use tokio::sync::mpsc;

mod bot;
mod panel;

#[launch]
async fn rocket() -> _ {
    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(async {
        bot::create_bot(rx).await;
    });

    rocket::build()
        .manage(tx)
        .mount("/", routes![panel::index, panel::send_message])
}
