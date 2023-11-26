use std::env;
use serenity::{async_trait, Client};
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler, GatewayIntents};

#[group]
#[commands(say)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

pub async fn create_bot() {
    let mut framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework = framework.configure(|c| c.prefix("!"));

    let token = env::var("DISCORD_TOKEN").expect("No token given in env");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error while creating client");

    if let Err(e) = client.start().await {
        println!("Error while running client: {:?}", e);
    }
}

#[command]
async fn say(ctx: &Context, msg: &Message) -> CommandResult {
    let content = msg.content.clone();
    let mut words = content.split_whitespace();
    let _ = words.next();
    let word = words.next();

    match word {
        Some(w) => {
            _ = msg.reply(ctx, w).await;
            Ok(())
        },
        None => {
            _ = msg.reply(ctx, "Please use a word!").await;
            Ok(())
        }
    }
}