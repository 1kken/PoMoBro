use serenity::model::channel::Message;

pub fn message_builder(msg: &Message) -> String{
    msg.author.name.to_owned()
} 
