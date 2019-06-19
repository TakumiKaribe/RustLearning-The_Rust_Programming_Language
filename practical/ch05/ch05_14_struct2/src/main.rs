#[derive(Default)]
struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

fn main() {
    let polygon1: Polygon = Default::default();

    let polygon2 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        .. Default::default()
    };
}
