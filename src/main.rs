use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::env;
//async timer imports
use tokio::time::{sleep, Duration};
//toDo make a struct for the pomodoro session then  make a function that accepts the struct
struct PomodoroDetails {
    focus_time: u32,
    rest_time: u32,
    long_rest_time: u32,
    number_sessions: u32,
}
//set the vectors value to respected value to build dynamic active[0]|rest[1]|long[2]|session[3]
impl PomodoroDetails {
    fn new(raw_data: Vec<&str>) -> Result<PomodoroDetails, ()> {
        let mut processed_data: Vec<u32> = Vec::new();
        for i in raw_data.iter() {
            if let Ok(data) = i.parse() {
                processed_data.push(data);
            } else {
                return Err(());
            }
        } //end loop
        Ok(PomodoroDetails {
            focus_time: processed_data[0],
            rest_time: processed_data[1],
            long_rest_time: processed_data[2],
            number_sessions: processed_data[3],
        })
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!start") {
            let show_guide = MessageBuilder::new()
                .push_bold(
                    "How to use\n
                *time measurement is in minutes*\n
                !start focus time | rest time | long rest time | number of sessions\n
                !start 30 5 60 4\n
                where\n
                    - 30 is focus minutes/learning time\n
                    - 5 is rest/break minutes\n
                    - 60 is the long rest/break minutes\n
                    - 4 is the number of rounds/session\n",
                )
                .build();
            let user_input = msg.content.trim().strip_prefix("!start "); // remove start
            let input_info: Vec<&str>;
            match user_input {
                Some(input) => {
                    //log inputs
                    println!("{}", input);
                    input_info = input.trim().split(' ').collect();
                    if let Ok(data) = PomodoroDetails::new(input_info) {
                        let active_session = Duration::new((data.focus_time * 60) as u64, 0);
                        let rest_session = Duration::new((data.rest_time * 60) as u64, 0);
                        let long_rest_session = Duration::new((data.long_rest_time * 60) as u64, 0);
                        //message builder
                        let mut session: u32 = 0;
                        let start_response = MessageBuilder::new()
                            .push(&msg.author)
                            .push("active session starts..")
                            .build();
                        let rest_response = MessageBuilder::new()
                            .push(&msg.author)
                            .push(" Active session ends time to rest..")
                            .build();
                        let long_rest_response = MessageBuilder::new()
                            .push(&msg.author)
                            .push(" Session finished long rest..")
                            .build();
                        let end_response = MessageBuilder::new()
                            .push(&msg.author)
                            .push(" End Session!")
                            .build();

                        //pomodoro main process
                        while session != data.number_sessions {
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
                            if session == data.number_sessions {
                                if let Err(_) = msg.reply_ping(&ctx.http, &long_rest_response).await
                                {
                                    println!("Error");
                                }
                                sleep(long_rest_session).await;
                                if let Err(_) = msg.reply_ping(&ctx.http, &end_response).await {
                                    println!("Error");
                                }
                            }
                        } //while loop end
                    } else {
                        if let Err(_) = msg.reply_mention(&ctx.http, "Invalid input").await {
                            println!("User input is wrong");
                        }
                    }
                }
                None => {
                    if let Err(e) = msg.reply_mention(&ctx.http, &show_guide).await {
                        println!("An error has occured log:{}", e);
                    }
                } //end none
            } //end match
        } //msg !start
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
