pub trait StrokeVisualizerTrait {
    fn draw(get_hand_position_for_ms: impl Fn(i32) -> f32) {
        get_hand_position_for_ms(0);
        println!("DRAWING");
    }
}

pub enum StrokeVisualizerImpl {}

impl StrokeVisualizerTrait for StrokeVisualizerImpl {}
