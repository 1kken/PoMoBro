use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::env;

/*timer imports
use std::thread;
use std::time::Duration;*/

//async timer imports
use tokio::time::{sleep, Duration};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let five_secs = Duration::new(5, 0);
        if msg.content == "!start" {
            let start_response = MessageBuilder::new()
                .push(&msg.author)
                .push(" test times starts now")
                .build();
            let end_response = MessageBuilder::new()
                .push(&msg.author)
                .push(" 5 secs passed timer done")
                .build();
            if let Err(why) = msg.reply_ping(&ctx.http, &start_response).await {
                println!("Error sending message: {:?}", why);
            }
            sleep(five_secs).await;
            if let Err(_) = msg.reply_ping(&ctx.http, &end_response).await {
                println!("Error");
            }
        } //msg start

        if msg.content == "!stats" {
            if let Err(_) = msg.reply_ping(&ctx.http, "Under development").await {
                println!("Error");
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
