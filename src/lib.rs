use crate::impure::hub;
use crate::impure::hub::AppConfigTrait;
use wasm_bindgen::prelude::*;

mod impure;
mod pure;

#[wasm_bindgen]
pub fn start() {
    struct AppConfig {};

    impl AppConfigTrait for AppConfig {
        type MessageHandler = crate::pure::message_handler::MessageHandlerImpl;
        type StrokeTimer = crate::pure::stroke_timer::StrokeTimerImpl;
        type StrokeVisualizer = crate::impure::stroke_visualizer::StrokeVisualizerImpl;
        type WebSocketAdaptor = crate::impure::websocket_adaptor::WebSocketAdaptorImpl;
    }

    hub::init::<AppConfig>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_start() {
        start();
    }

}
