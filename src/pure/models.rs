use lazy_static::lazy_static;

pub struct Message {}

pub struct StrokeSettings {
    pub pattern: &'static StrokePattern,
    pub strokes_per_minute: i32,
}

pub struct StrokePattern {
    pub strokes_per_cycle: i32,
    pub path: Vec<(i32, i32)>,
}

lazy_static! {
    pub static ref DEFAULT_PATTERN: StrokePattern = StrokePattern {
        strokes_per_cycle: 1,
        path: vec![(0, 0), (10, 100), (20, 0)],
    };
}
