use crate::pure::models::StrokeSettings;

pub trait StrokeTimerTrait {
    fn hand_position(_settings: &StrokeSettings, _milliseconds_since_start: i32) -> f32 {
        1.1
    }
}

pub enum StrokeTimerImpl {}

impl StrokeTimerTrait for StrokeTimerImpl {}
