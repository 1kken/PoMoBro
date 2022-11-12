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
    use tokio::sync::mpsc::{self, Receiver, Sender};
    use tokio::time::{sleep, Duration};
    //import for the ferson
    use serenity::model::user::User;
    use std::collections::HashMap;
    use std::sync::Mutex;
    //we need lazy static for shared data
    lazy_static! {
        static ref USER_POOL: Mutex<HashMap<User,&'static str>> = Mutex::new(HashMap::new());
    }
    //why theres function in here well idk but std::Mutex can be use in async functions
    fn add_usr(usr: User){ 
        let mut user_pool = USER_POOL.lock().unwrap();
        user_pool.insert(usr, "test");
    }

    fn contains(usr: &User) -> bool {
        let user_pool = USER_POOL.lock().unwrap();
        if user_pool.contains_key(usr) {
            true
        } else {
            false
        }
    }

    pub fn stop_client(msg: &Message){
        let mut user_pool = USER_POOL.lock().unwrap();
        user_pool.remove(&msg.author); 
    }

    pub async fn main_handler(ctx: Context, message: &Message) {
        add_client(data_parser(&ctx, &message)).await;
    }


    async fn add_client<'a>(det: Data<'a>) {
        let (tx, mut rx): (Sender<&str>, Receiver<&str>) = mpsc::channel(100);
        add_usr(det.msg.author.clone()); 
        tokio::spawn(async move {
            let mut ctr = det.session;
            while ctr > 0{
                tx.send("!start").await.unwrap();
                sleep(Duration::from_secs(det.active as u64 * 60)).await;

                tx.send("!rest").await.unwrap();
                sleep(Duration::from_secs(det.rest as u64 * 60)).await;

                tx.send("!long_rest").await.unwrap();
                sleep(Duration::from_secs(det.long_rest as u64 * 60)).await;
                ctr -= 1;
            }
            tx.send("!done").await.unwrap();
        });
        while let Some(i) = rx.recv().await {
            notify_client(i, &det).await;
        }
    }


    use crate::msg::MessageType::{Done,LngRest,Rest};
    async fn notify_client<'a>(to_send: &str, det: &Data<'a>) {
        let message = det.msg;
        let ctx = det.ctx; 
        let active = contains(&det.msg.author);
        match to_send {
            "!start" if active => {
                if let Err(why) = &message.reply_ping(&ctx, "**Focus Now**").await {
                    println!("Error sending message: {:?}", why);
                };
            }
            "!rest" if active  => {
                if let Err(why) = &message
                    .reply_ping(&ctx, msg::message_builder(&message, Rest))
                    .await
                {
                    println!("Error sending message: {:?}", why);
                };
            }
            "!long_rest" if active  => {
                if let Err(why) = &message
                    .reply_ping(&ctx, msg::message_builder(&message, LngRest))
                    .await
                {
                    println!("error sending message: {:?}", why);
                };
            }
            "!done" if active  => {
                   stop_client(&message); 
                    if let Err(why) = &message
                        .reply_ping(&ctx, msg::message_builder(&message, Done))
                        .await
                    {
                        println!("error sending message: {:?}", why);
                    }
            }
            _  => { }
        }
    }
}

pub mod msg {
    use super::*;
    use chrono::prelude::*;

    pub enum MessageType {
        Help,
        Start,
        Rest,
        Stop,
        LngRest,
        Done,
    }

    use MessageType::{Done, Help, LngRest, Rest, Start, Stop};
    pub fn message_builder(_msg: &Message, msg_type: MessageType) -> String {
        let now = Local::now();
        let time = format!("{}:{}", now.hour(), now.minute());
        match msg_type {
            Help => {
                MessageBuilder::new()
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
                    .build()
            }
            Start => {
                MessageBuilder::new()
                    .push_bold("Pomodoro started @ ")
                    .push(" ")
                    .push_bold(time)
                    .build()
            }
            Rest => {
                 MessageBuilder::new()
                    .push_bold("Pomodoro rest @ ")
                    .push(" ")
                    .push_bold(time)
                    .build()
            }
            Stop => {
                MessageBuilder::new()
                    .push_bold("Pomodoro stopped @")
                    .push(" ")
                    .push_bold(time)
                    .build()
            }
            LngRest => {
                MessageBuilder::new().push_bold("Long rest").build()
            }
            Done => {
                MessageBuilder::new()
                    .push_bold("Congrats your DONE!")
                    .build()
            }
        }
    }
}
