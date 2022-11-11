use serenity::model::channel::Message;
pub mod msg {
    use super::*;
    pub fn message_builder(msg: &Message) -> String {
        msg.author.name.to_owned()
    }
}

pub mod parsing {
    use super::*;

    pub fn inp_parser(msg: &Message, activator: &str) -> bool {
        //WE need to process the input to make sure it can be processed
        let elements: Vec<&str> = msg.content.split(' ').collect();
        elements.contains(&activator)
    }
}
