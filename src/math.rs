use std::iter::Sum;
use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn up() -> Vec3 {
        Vec3(0.0, 1.0, 0.0)
    }

    pub fn down() -> Vec3 {
        Vec3(0.0, -1.0, 0.0)
    }

    pub fn front() -> Vec3 {
        Vec3(0.0, 0.0, 1.0)
    }

    pub fn back() -> Vec3 {
        Vec3(0.0, 0.0, -1.0)
    }

    pub fn left() -> Vec3 {
        Vec3(1.0, 0.0, 0.0)
    }

    pub fn right() -> Vec3 {
        Vec3(-1.0, 0.0, 0.0)
    }

    pub fn x(self) -> f32 {
        self.0
    }

    pub fn y(self) -> f32 {
        self.1
    }

    pub fn z(self) -> f32 {
        self.2
    }

    pub fn normalized(self) -> Vec3 {
        self / self.len()
    }

    pub fn len_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn len(self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn dot(self, v: Vec3) -> f32 {
        self.x() * v.x() +
            self.y() * v.y() +
            self.z() * v.z()
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3(
            self.y() * v.z() - self.z() * v.y(),
            self.z() * v.x() - self.x() * v.z(),
            self.x() * v.y() - self.y() * v.x(),
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
        )
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Vec3::zero(), |acc, i| acc + i)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
        )
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(
            -self.x(),
            -self.y(),
            -self.z(),
        )
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z(),
        )
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(
            self * rhs.x(),
            self * rhs.y(),
            self * rhs.z()
        )
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3(
            self.x() / rhs.x(),
            self.y() / rhs.y(),
            self.z() / rhs.z(),
        )
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs
        )
    }
}

#[derive(Debug, Clone)]
pub struct Mat4x4(pub [f32; 16]);

impl Mat4x4 {
    pub fn identity() -> Self {
        let mut m = [0.0; 16];
        m[0] = 1.0;
        m[5] = 1.0;
        m[10] = 1.0;
        m[15] = 1.0;
        Self(m)
    }

    pub fn translate(&mut self, v: Vec3) {
        let mut m = Mat4x4::identity();
        m.0[3] = v.x();
        m.0[7] = v.y();
        m.0[11] = v.z();
        m *= self.clone();
        *self = m;
    }

    pub fn scale(&mut self, v: Vec3) {
        let mut m = Mat4x4::identity();
        m.0[0] = v.x();
        m.0[5] = v.y();
        m.0[10] = v.z();
        m *= self.clone();
        *self = m;
    }

    pub fn rotate(&mut self, d: f32, v: Vec3) {
        let d = radians(d);
        let mut m = Mat4x4::identity();
        m.0[0] = v.x() * v.x() * (1.0 - d.cos()) + d.cos();
        m.0[1] = v.y() * v.x() * (1.0 - d.cos()) - v.z() * d.sin();
        m.0[2] = v.z() * v.x() * (1.0 - d.cos()) + v.y() * d.sin();
        m.0[4] = v.x() * v.y() * (1.0 - d.cos()) + v.z() * d.sin();
        m.0[5] = v.y() * v.y() * (1.0 - d.cos()) + d.cos();
        m.0[6] = v.z() * v.y() * (1.0 - d.cos()) - v.x() * d.sin();
        m.0[8] = v.x() * v.z() * (1.0 - d.cos()) - v.y() * d.sin();
        m.0[9] = v.y() * v.z() * (1.0 - d.cos()) + v.x() * d.sin();
        m.0[10] = v.z() * v.z() * (1.0 - d.cos()) + d.cos();
        m *= self.clone();
        *self = m;
    }
}

impl MulAssign for Mat4x4 {
    fn mul_assign(&mut self, rhs: Self) {
        let a = self.0;
        let b = rhs.0;
        let mut c = [0.0; 16];
        for y in 0..4 {
            for x in 0..4 {
                for i in 0..4 {
                    c[y * 4 + x] += a[4 * y + i] * b[4 * i + x];
                }
            }
        }
        self.0 = c;
    }
}

pub struct Camera {
    pub view: Mat4x4,
    pub projection: Mat4x4,
}

impl Camera {
    pub fn new(aspect: f32, fov: f32, near: f32, far: f32) -> Self {
        let mut projection = Mat4x4::identity();

        projection.0[0] = 1.0 / (aspect * (fov * 0.5).tan());
        projection.0[5] = 1.0 / (fov * 0.5).tan();
        projection.0[10] = -(far + near) / (far - near);
        projection.0[11] = -(2.0 * far * near) / (far - near);
        projection.0[14] = -1.0;
        projection.0[15] = 0.0;

        Self {
            view: Mat4x4::identity(),
            projection,
        }
    }
}

pub fn radians(degree: f32) -> f32 {
    std::f32::consts::PI * degree / 180.0
}