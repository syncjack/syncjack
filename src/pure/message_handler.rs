use crate::pure::models::Message;
use crate::pure::models::StrokeSettings;

pub trait MessageHandlerTrait {
    fn process(_message: Message) -> StrokeSettings {
        StrokeSettings {}
    }
}

pub struct MessageHandlerImpl {}

impl MessageHandlerTrait for MessageHandlerImpl {}
