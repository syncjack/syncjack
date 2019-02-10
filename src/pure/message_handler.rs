use crate::pure::models::DEFAULT_PATTERN;
use crate::pure::models::Message;
use crate::pure::models::StrokeSettings;

pub trait MessageHandlerTrait {
    fn process(_message: Message) -> StrokeSettings {
        StrokeSettings {
            pattern: &*DEFAULT_PATTERN,
            strokes_per_minute: 60,
        }
    }
}

pub struct MessageHandlerImpl {}

impl MessageHandlerTrait for MessageHandlerImpl {}
