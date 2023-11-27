use std::env;
use rocket::yansi::Paint;
use serenity::builder::CreateMessage;
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
                MessageType::UploadFile => {
                    let data = msg.1.expect("Upload requires data!");
                    let mut data_split = data.split(":");
                    let file_path = data_split.next().expect("No file path").to_string();
                    let file_name = data_split.next().expect("No file name").to_string();
                    send_message_to_discord(
                        &client,
                        file_path,
                        file_name
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

async fn send_message_to_discord(client: &Client, file_path: String, file_name: String) {
    let channel = ChannelId(env::var("CHANNEL_ID").expect("No channel id given in env").parse().unwrap());
    let f = [(&tokio::fs::File::open(file_path.as_str()).await.expect("No file"), file_name.as_str())];
    channel.send_message(&client.cache_and_http.http, |m| {
        m.content("");
        m.files(f);
        return m;
    }).await.expect("Failed to upload file");
    tokio::fs::remove_file(file_path.as_str()).await.expect("Failed to remove file");
}
