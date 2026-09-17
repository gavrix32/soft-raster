#[derive(Copy, Clone)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
}

impl Pixel {
    pub fn new(x: usize, y: usize) -> Pixel {
        Self { x, y }
    }
}

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

// impl Vec2 {
//     fn new(x: f32, y: f32) -> Vec2 {
//         Self { x, y }
//     }
// }

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Self { x, y, z }
    }
}
