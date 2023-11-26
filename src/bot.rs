use std::env;
use tokio::sync::mpsc::Receiver;
use serenity::Client;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::model::id::ChannelId;
use serenity::prelude::GatewayIntents;

#[group]
struct General;

pub async fn create_bot(mut rx: Receiver<String>) {
    let mut framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework = framework.configure(|c| c.prefix("!"));

    let token = env::var("DISCORD_TOKEN").expect("No token given in env");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(token, intents)
        .framework(framework)
        .await
        .expect("Error while creating client");

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Received message in bot: {}", msg);
            send_message_to_discord(&client, msg).await;
        }
        println!("Bot receiver channel closed");
    });
}

async fn send_message_to_discord(client: &Client, msg: String) {
    let channel = ChannelId(env::var("CHANNEL_ID").expect("No channel id given in env").parse().unwrap());

    if let Err(e) = channel.say(&client.cache_and_http.http, msg).await {
        println!("Error sending message: {:?}", e);
    }
}
