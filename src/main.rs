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
        let active_session = Duration::new(10, 0);
        let rest_session = Duration::new(5, 0);
        if msg.content == "!start" {
            let mut session: u8 = 0;
            let start_response = MessageBuilder::new()
                .push(&msg.author)
                .push("active session starts..")
                .build();
            let rest_response = MessageBuilder::new()
                .push(&msg.author)
                .push(" Active session ends time to rest..")
                .build();
            let end_response = MessageBuilder::new()
                .push(&msg.author)
                .push(" End Session!")
                .build();
            //A basic and traditional pomodoro contains 4 iterations of the methods eg.. active-rest 1 | active-rest 2 | active-rest 3 | active-rest 4
            while session != 4 {
                if let Err(why) = msg.reply_ping(&ctx.http, &start_response).await {
                    println!("Error sending message: {:?}", why);
                }
                //timer for the current active/pomodoro session
                sleep(active_session).await;
                //timer for the rest_session
                if let Err(_) = msg.reply_ping(&ctx.http, &rest_response).await {
                    println!("Error");
                }
                sleep(rest_session).await;
                session += 1;
                //end session
                if session == 4 {
                    if let Err(_) = msg.reply_ping(&ctx.http, &end_response).await {
                        println!("Error");
                    }
                }
            } //while loop end
        } //msg start

        if msg.content == "!stats" {
            if let Err(_) = msg.reply_ping(&ctx.http, "Under development").await {
                println!("Error");
            }
        } //msg stats
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
