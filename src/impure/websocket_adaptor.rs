use crate::pure::models::Message;

pub trait WebSocketAdaptorTrait {
    fn connect(url: &str, mut on_incoming_message: impl FnMut(Message)) {
        url.bytes();
        on_incoming_message(Message {});
    }
}

pub struct WebSocketAdaptorImpl {}

impl WebSocketAdaptorTrait for WebSocketAdaptorImpl {}
