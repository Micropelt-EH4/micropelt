pub enum Port {
    Operate = 1,
    Version = 2,
    Motor = 3,
    DataRate = 4,
}

pub fn close(x: f32, y: f32, resolution: f32) -> bool {
    (x - y) * 2.0 < resolution && (y - x) * 2.0 < resolution
}
