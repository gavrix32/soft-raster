use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vec2U {
    pub x: usize,
    pub y: usize,
}

impl Vec2U {
    pub fn new(x: usize, y: usize) -> Vec2U {
        Self { x, y }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Vec2I {
    pub x: i32,
    pub y: i32,
}

impl Vec2I {
    pub fn new(x: i32, y: i32) -> Vec2I {
        Self { x, y }
    }

    pub fn as_vec2u(self) -> Vec2U {
        Vec2U::new(self.x as usize, self.y as usize)
    }

    pub fn cross(self, other: Self) -> i32 {
        self.x * other.y - self.y * other.x
    }
}

impl Sub for Vec2I {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<i32> for Vec2I {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self::new(self.x - rhs, self.y - rhs)
    }
}

impl Add<i32> for Vec2I {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs)
    }
}

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Self { x, y }
    }

    pub fn splat(v: f32) -> Self {
        Self::new(v, v)
    }

    pub fn from_array(a: [f32; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn splat(v: f32) -> Self {
        Self::new(v, v, v)
    }

    pub fn zero() -> Self {
        Self::splat(0.0)
    }

    pub fn from_array(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    pub fn dot(self, v: Self) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn len(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let len = self.len();
        if len > f32::EPSILON {
            self / len
        } else {
            Self::zero()
        }
    }

    pub fn cross(self, v: Self) -> Self {
        Self::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[derive(Copy, Clone)]
pub struct Vec3I {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3I {
    pub fn new(x: i32, y: i32, z: i32) -> Vec3I {
        Self { x, y, z }
    }
}

#[derive(Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_array(v: [f32; 4]) -> Self {
        Self::new(v[0], v[1], v[2], v[3])
    }

    pub fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

#[derive(Copy, Clone)]
pub struct Mat4 {
    pub m: [f32; 16],
}

impl Mat4 {
    // pub fn identity() -> Self {
    //     Self {
    //         #[rustfmt::skip]
    //         m: [
    //             1.0, 0.0, 0.0, 0.0,
    //             0.0, 1.0, 0.0, 0.0,
    //             0.0, 0.0, 1.0, 0.0,
    //             0.0, 0.0, 0.0, 1.0,
    //         ],
    //     }
    // }

    // pub fn zero() -> Self {
    //     Self { m: [0.0; 16] }
    // }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();
        Self {
            #[rustfmt::skip]
            m: [
                f / aspect, 0.0, 0.0,                         0.0,
                0.0,        f,   0.0,                         0.0,
                0.0,        0.0, (far + near) / (near - far), (2.0 * far * near) / (near - far),
                0.0,        0.0, -1.0,                        0.0,
            ],
        }
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = (target - eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        Self {
            #[rustfmt::skip]
            m: [
                s.x,  s.y,  s.z,  -s.dot(eye),
                u.x,  u.y,  u.z,  -u.dot(eye),
                -f.x, -f.y, -f.z, f.dot(eye),
                0.0,  0.0,  0.0,  1.0,
            ],
        }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let v_arr = rhs.to_array();
        let mut result = [0.0; 4];

        for i in 0..4 {
            let mut sum = 0.0;
            for j in 0..4 {
                sum += self.m[i * 4 + j] * v_arr[j];
            }
            result[i] = sum;
        }
        Vec4::from_array(result)
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = [0.0; 16];

        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.m[i * 4 + k] * rhs.m[k * 4 + j];
                }
                m[i * 4 + j] = sum;
            }
        }
        Self { m }
    }
}

pub fn bary_lerp<T>(bary: Vec3, v0: T, v1: T, v2: T) -> T
where
    T: Copy + Add<Output = T> + Mul<f32, Output = T>,
{
    v0 * bary.x + v1 * bary.y + v2 * bary.z
}
