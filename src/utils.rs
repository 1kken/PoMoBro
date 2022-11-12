use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
pub struct Data<'a> {
    active: usize,
    rest: usize,
    long_rest: usize,
    session: usize,
    ctx: &'a Context,
    msg: &'a Message,
}

pub mod parsing {
    use super::*;

    pub fn inp_parser(msg: &Message, _activator: &str) -> bool {
        //WE need to process the input to make sure it can be processed
        let elements: Vec<&str> = msg.content.trim().split(' ').collect();
        if _activator == "!start" && elements.len() == 5 && elements.contains(&_activator) {
            true
        } else if elements.contains(&_activator) && _activator != "!start" {
            true
        } else {
            false
        }
    }

    pub fn data_parser<'a>(ctx: &'a Context, msg: &'a Message) -> Data<'a> {
        let data: Vec<String> = msg
            .content
            .split(" ")
            .map(|data| data.to_string())
            .collect();
        Data {
            active: data[1].parse::<usize>().unwrap(),
            rest: data[2].parse::<usize>().unwrap(),
            long_rest: data[3].parse::<usize>().unwrap(),
            session: data[4].parse::<usize>().unwrap(),
            ctx,
            msg,
        }
    }
}

pub mod client_handler {
    use super::parsing::data_parser;
    use super::*;
    use crate::msg::MessageType::{Help, Rest, Start, Stop};
    use tokio::sync::mpsc::{self, Receiver, Sender};
    use tokio::time::{sleep, Duration};
    // use std::collections::HashMap;
    // use std::sync::{Mutex,Arc};
    // use serenity::model::user::User;
    //we need to have a hashmap that can handle concurrency which means Arc<Mutex<T>>>

    pub async fn main_handler(ctx: Context, message: &Message) {
        add_client(data_parser(&ctx, &message)).await;
    }

    async fn add_client<'a>(det: Data<'a>) {
        let (tx, mut rx): (Sender<&str>, Receiver<&str>) = mpsc::channel(100);
        tokio::spawn(async move {
            let mut ctr = det.session;
            while ctr > 0 {
                tx.send("!start").await.unwrap();
                sleep(Duration::from_secs(det.active as u64)).await;
                tx.send("!rest").await.unwrap();
                sleep(Duration::from_secs(det.rest as u64)).await;
                tx.send("!long_rest").await.unwrap();
                sleep(Duration::from_secs(det.long_rest as u64)).await;
                ctr -= 1;
            }
        });
        let message = det.msg;
        let ctx = det.ctx;
        while let Some(i) = rx.recv().await {
            match i {
                "!start" => {
                    if let Err(why) = &message
                        .reply_ping(&ctx, "Focus now!")
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    };
                }
                "!rest" => {
                    if let Err(why) = &message
                        .reply_ping(&ctx, msg::message_builder(&message, Rest))
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    };
                }
                "!long_rest" => {
                    if let Err(why) = &message
                        .reply_ping(&ctx, msg::message_builder(&message, Rest))
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    };
                }
                _ => {
                    if let Err(why) = &message
                        .reply_ping(&ctx, msg::message_builder(&message, Help))
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    };
                }
            }
        }
    }

    // fn stop_timer(){}

    // fn notify_client(){}
}

pub mod msg {
    use super::*;
    use chrono::prelude::*;

    pub enum MessageType {
        Help,
        Start,
        Rest,
        Stop,
    }
    use MessageType::{Help, Rest, Start, Stop};
    pub fn message_builder(msg: &Message, msg_type: MessageType) -> String {
        let mut response = String::new();
        let now = Local::now();
        let time = format!("{}:{}", now.hour(), now.minute());
        match msg_type {
            Help => {
                response = MessageBuilder::new()
                    .push_bold_line("How to use")
                    .push_bold_line("Time measurement is in minutes")
                    .push_mono_line(
                        "!start focus time | rest time | long rest time | number of sessions",
                    )
                    .push_mono_line("!start 30 5 60 4")
                    .push_italic_line(
                        "-30 is focus minutes/learning time  -5 is rest/break minutes",
                    )
                    .push_italic_line(
                        "-60 is the long rest/break minutes- 4 is the number of rounds/session",
                    )
                    .build();
            }
            Start => {
                response = MessageBuilder::new()
                    .push_bold("Pomodoro started @ ")
                    .push(" ")
                    .push_bold(time)
                    .build();
            }
            Rest => {
                response = MessageBuilder::new()
                    .push_bold("Pomodoro rest @ ")
                    .push(" ")
                    .push_bold(time)
                    .build();
            }
            Stop => {
                response = MessageBuilder::new()
                    .push_bold("Pomodoro stopped @")
                    .push(" ")
                    .push_bold(time)
                    .build();
            }
        }
        response
    }
}
