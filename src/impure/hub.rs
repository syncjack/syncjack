use crate::impure::stroke_visualizer::StrokeVisualizerTrait;
use crate::impure::websocket_adaptor::WebSocketAdaptorTrait;
use crate::pure::message_handler::MessageHandlerTrait;
use crate::pure::models::DEFAULT_PATTERN;
use crate::pure::models::Message;
use crate::pure::models::StrokeSettings;
use crate::pure::stroke_timer::StrokeTimerTrait;

pub trait AppConfigTrait {
    type MessageHandler: MessageHandlerTrait;
    type StrokeTimer: StrokeTimerTrait;
    type StrokeVisualizer: StrokeVisualizerTrait;
    type WebSocketAdaptor: WebSocketAdaptorTrait;
}

pub fn init<C: AppConfigTrait>() {
    let mut settings = StrokeSettings {
        pattern: &*DEFAULT_PATTERN,
        strokes_per_minute: 60,
    };

    let on_incoming_message = |message: Message| {
        settings = C::MessageHandler::process(message);
    };
    C::WebSocketAdaptor::connect("", on_incoming_message);

    let get_hand_position_for_ms = |milliseconds_since_start: i32| -> f32 {
        C::StrokeTimer::hand_position(&settings, milliseconds_since_start)
    };
    for _ in 0..10 {
        C::StrokeVisualizer::draw(get_hand_position_for_ms);
    }
    dbg!(C::StrokeVisualizer::make_svg());
}
