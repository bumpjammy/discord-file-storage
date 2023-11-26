use std::env;
use tokio::sync::mpsc::Receiver;
use serenity::Client;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::futures::StreamExt;
use serenity::model::id::ChannelId;
use serenity::prelude::GatewayIntents;
use crate::message::{Message, MessageChannel, MessageType};

#[group]
struct General;

pub async fn create_bot(mut channel: MessageChannel) {
    let mut framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework = framework.configure(|c| c.prefix("!"));

    let token = env::var("DISCORD_TOKEN").expect("No token given in env");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(token, intents)
        .framework(framework)
        .await
        .expect("Error while creating client");

    tokio::spawn(async move {
        while let Some(msg) = channel.0.lock().await.recv().await {
            match msg.0 {
                MessageType::SendMessage => {
                    send_message_to_discord(
                        &client,
                        msg.1.expect("Send message requires data!")
                    ).await
                },
                MessageType::GetFiles => {
                    let channelID = ChannelId(env::var("CHANNEL_ID").expect("No channel id given in env").parse().unwrap());
                    let response = get_files_from_channel(&client, channelID).await;

                    channel.1.send(
                        Message (
                            MessageType::GetFiles,
                            Some(response)
                        )
                    )
                    .await
                    .expect("Failed to return files");
                }
            }
        }
        println!("Bot receiver channel closed");
    });
}

async fn get_files_from_channel(client: &Client, channel: ChannelId) -> String {
    let mut result = "".to_string();
    let mut messages = channel.messages_iter(&client.cache_and_http.http).boxed();
    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => {
                if message.attachments.is_empty() {
                    continue
                }

                let filename = message.attachments.get(0).expect("?").filename.as_str();
                let url = message.attachments.get(0).expect("?").url.as_str();
                result.push_str("<tr>");
                result.push_str(format!("{}{}{}", "<td>", filename, "</td>").as_str());
                result.push_str(format!("{}{}{}", "<td><a href=\"", url, "\">Download Link</td>").as_str());
                result.push_str("<tr>");
            },
            Err(error) => eprintln!("Uh oh! Error: {}", error),
        }
    }
    result
}

async fn send_message_to_discord(client: &Client, msg: String) {
    let channel = ChannelId(env::var("CHANNEL_ID").expect("No channel id given in env").parse().unwrap());

    if let Err(e) = channel.say(&client.cache_and_http.http, msg).await {
        println!("Error sending message: {:?}", e);
    }
}
