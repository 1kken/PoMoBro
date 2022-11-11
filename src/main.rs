mod utils;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use utils::{msg, parsing};

struct Handler;
use utils::msg::MessageType::{Help,Start,Rest,Stop};

#[async_trait]
impl EventHandler for Handler {
    //this method let you do some epic things
    async fn message(&self, ctx: Context, msg: Message) {
        if parsing::inp_parser(&msg, "!start") {
            if let Err(why) = msg
                .reply_ping(&ctx, msg::message_builder(&msg, Start))
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }

        if parsing::inp_parser(&msg, "!help") {
            if let Err(why) = msg
                .reply_ping(&ctx, msg::message_builder(&msg, Help))
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }

    }

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

