use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

pub trait StrokeVisualizerTrait {
    fn draw(get_hand_position_for_ms: impl Fn(i32) -> f32) {
        get_hand_position_for_ms(0);
        println!("DRAWING");
    }

    fn make_svg() -> String {
        let data = Data::new()
            .move_to((10, 10))
            .line_by((0, 50))
            .line_by((50, 0))
            .line_by((0, -50))
            .close();

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", data);

        let document = Document::new().set("viewBox", (0, 0, 100, 100)).add(path);
        document.to_string()
    }
}

pub enum StrokeVisualizerImpl {}

impl StrokeVisualizerTrait for StrokeVisualizerImpl {}
