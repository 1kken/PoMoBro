use serenity::model::channel::Message;

pub mod parsing {
    use super::*;

    pub fn inp_parser(msg: &Message, activator: &str) -> bool {
        //WE need to process the input to make sure it can be processed
        let elements: Vec<&str> = msg.content.split(' ').collect();
        elements.contains(&activator)
    }
}


pub mod msg {
    use super::*;
    pub enum MessageType{
        Help,
        Start,
        Rest,
        Stop,
    }
    use MessageType::{Rest,Help,Start,Stop};
    pub fn message_builder(msg: &Message,msg_type: MessageType) -> String {
        match msg_type{
            Help => "Help me".to_string(),
            Start => "Started".to_string(),
            Rest => "Rested".to_string(),
            Stop => "Stopped".to_string(),
        }
    }
}
