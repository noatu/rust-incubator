#[derive(Default, Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
struct Polyline {
    start: Point, // **non-empty** set should always have one point
    points: Vec<Point>,
}

fn main() {
    println!("Implement me!");
}
