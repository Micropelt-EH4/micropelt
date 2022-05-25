pub enum Port {
    Operate = 1,
}

pub fn close(x: f32, y: f32, resolution: f32) -> bool {
    (x - y) * 2.0 < resolution && (y - x) * 2.0 < resolution
}
