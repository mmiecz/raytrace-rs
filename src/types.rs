use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

type Point = Vec3;

pub struct Color {
    rgb: [u8; 3],
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { rgb: [r, g, b] }
    }

    pub fn as_array(&self) -> [u8; 3] {
        self.rgb
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Point,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + self.dir * t
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3([f32; 3]);

impl Vec3 {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Vec3 { 0: [a, b, c] }
    }
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn length(&self) -> f32 {
        let len_squared = self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2];
        len_squared.sqrt()
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1] + self.0[2] * rhs.0[2]
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Vec3::new(
            self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
            self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
            self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
        )
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }
}

//TODO: Move this to macro
// Op<Vec3> for `Scalar`
// Op<`Scalar`> for Vec3
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            0: [
                self.0[0] - rhs.0[0],
                self.0[1] - rhs.0[1],
                self.0[2] - rhs.0[2],
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0[0] -= rhs.0[0];
        self.0[1] -= rhs.0[1];
        self.0[2] -= rhs.0[2];
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            0: [-self.0[0], -self.0[1], -self.0[2]],
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            0: [
                self.0[0] + rhs.0[0],
                self.0[1] + rhs.0[1],
                self.0[2] + rhs.0[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}
//TODO: more types than f32
impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            0: [self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs],
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            0: [self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs],
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0[0] /= rhs;
        self.0[1] /= rhs;
        self.0[2] /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    macro_rules! vec3_compare {
        ($v:expr, $exp:expr) => {
            let eps = f32::EPSILON;
            assert!(($v.x() - $exp[0]).abs() < eps);
            assert!(($v.y() - $exp[1]).abs() < eps);
            assert!(($v.z() - $exp[2]).abs() < eps);
        };
    }

    #[test]
    fn test_vec3() {
        let v = Vec3::new(0.0, 1.0, 2.0);
        vec3_compare!(v, [0.0, 1.0, 2.0]);
    }

    #[test]
    fn test_add() {
        let v = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(0.0, 1.0, 2.0);
        let result = v + v2;
        vec3_compare!(result, [0.0, 2.0, 4.0]);
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(-1.0, 1.0, 0.0);
        let neg_v = -v;
        vec3_compare!(neg_v, [1.0, -1.0, 0.0]);
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vec3::new(0.0, 1.0, 2.0);
        v += Vec3::new(1.0, 1.0, 1.0);
        vec3_compare!(v, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_mul() {
        let v = Vec3::new(0.0, 1.0, 5.0);
        let v = v * 3.0;
        vec3_compare!(v, [0.0, 3.0, 15.0]);
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec3::new(0.0, 1.0, 5.0);
        v *= 3.0;
        vec3_compare!(v, [0.0, 3.0, 15.0]);
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec3::new(0.0, 1.0, 5.0);
        v /= 2.0;
        vec3_compare!(v, [0.0, 0.5, 2.5]);
    }

    #[test]
    fn test_vec3_length() {
        let vec = Vec3::new(3.0, 4.0, 5.0);
        assert!((vec.length() - 50.0_f32.sqrt()).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vec3_dot() {
        let vec = Vec3::new(1.0, 3.0, -5.0);
        let vec2 = Vec3::new(4.0, -2.0, -1.0);
        let result = vec.dot(vec2);
        assert!((result - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vec3_cross() {
        let vec = Vec3::new(2.0, 3.0, 4.0);
        let vec2 = Vec3::new(5.0, 6.0, 7.0);
        let result = vec.cross(vec2);
        vec3_compare!(result, [-3.0, 6.0, -3.0]);
    }
}
