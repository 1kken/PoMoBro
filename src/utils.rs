use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;
pub struct data {
    active: usize,
    rest: usize,
    long_rest: usize,
    session: usize,
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

    pub fn data_parser(msg: &Message) -> data {
        let data: Vec<String> = msg
            .content
            .split(" ")
            .map(|data| data.to_string())
            .collect();
        data {
            active: data[1].parse::<usize>().unwrap(),
            rest: data[2].parse::<usize>().unwrap(),
            long_rest: data[3].parse::<usize>().unwrap(),
            session: data[4].parse::<usize>().unwrap(),
        }
    }
}

pub mod client_handler {
    use super::parsing::data_parser;
    use super::*;
    use std::thread;
    use std::time::Duration;
    // use std::collections::HashMap;
    // use std::sync::{Mutex,Arc};
    // use serenity::model::user::User;
    //we need to have a hashmap that can handle concurrency which means Arc<Mutex<T>>>
    pub fn main_handler(msg: &Message) {
        add_client(data_parser(msg));
    }

    fn add_client(det: data) {
        thread::spawn(move || {
            let mut session = det.session as u32;
            while session > 0 {
                //active session
                println!("Started");
                thread::sleep(Duration::from_secs(det.active as u64));
                //rest session
                thread::sleep(Duration::from_secs(det.rest as u64));
                println!("Rest");
                //long rest session
                thread::sleep(Duration::from_secs(det.long_rest as u64));
                println!("Long rest");
                session -= 1;
            }
        });
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
