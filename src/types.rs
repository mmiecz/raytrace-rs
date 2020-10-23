use nalgebra as na;
use std::ops::{Add, Mul, MulAssign, Neg};

pub type Vec3 = na::Vector3<f32>;
pub type Mat3 = na::Matrix3<f32>;
pub type Point3 = na::Point3<f32>;
pub type Point4 = na::Point4<f32>;
pub type Vec4 = na::Vector4<f32>;
pub type Mat4 = na::Matrix4<f32>;

//Pouint in 3D space with w component = 0
macro_rules! point{
    ($x:expr, $y:expr, $z:expr) => {
        Point4::new($x,$y,$z,1.0)
    }
}

//3D Vector with w component = 0
macro_rules! vector{
    ($x:expr, $y:expr, $z:expr) => {
        Vec4::new($x,$y,$z,0.0)
    }
}

//Translation matrix
macro_rules! translation{
    ($x:expr,$y:expr,$z:expr) => {
        nalgebra::Translation3::new($x,$y,$z).to_homogeneous()
    }
}

macro_rules! scaling{
    ($x:expr,$y:expr,$z:expr) => {
        nalgebra::Matrix4::new_nonuniform_scaling(&Vec3::new($x,$y,$z))
    }
}



#[derive(Copy, Clone)]
pub struct Color {
    rgb: Vec3,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color{ rgb: Vec3::new(r, g, b)}
    }

    pub fn r(&self) -> f32 {
        self.rgb[0]
    }

    pub fn g(&self) -> f32 {
        self.rgb[1]
    }

    pub fn b(&self) -> f32 {
        self.rgb[2]
    }

    pub fn as_array(&self) -> [f32; 3] {
        self.rgb.into()
    }

    pub fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Color {
        Color::new(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color{ rgb : self.rgb + rhs.rgb }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color{ rgb: self.rgb.component_mul(&rhs.rgb) }
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.rgb.component_mul_assign(&rhs.rgb);
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color{ rgb: rhs*self.rgb}
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, rhs: Color)  -> Self::Output {
        rhs * self
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! vec3_compare {
        ($vec:expr, $exp:expr) => {
            let eps = f32::EPSILON;
            assert!(($vec.x - $exp[0]).abs() < eps);
            assert!(($vec.y - $exp[1]).abs() < eps);
            assert!(($vec.z - $exp[2]).abs() < eps);
        };
    }
    #[test]
    fn vec3_creation() {
        let v = Vec3::new(0.0, 1.0, 2.0);
        vec3_compare!(v, [0.0, 1.0, 2.0]);
    }

    #[test]
    fn vec3_add() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        let result = v1 + v2;
        vec3_compare!(result, [3.0, 5.0, 7.0]);
    }

    #[test]
    fn vec3_sub() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        let result = v1 - v2;
        vec3_compare!(result, [-3.0, -3.0, -3.0]);
    }

    #[test]
    fn vec3_neg() {
        let v = Vec3::new(0.0, 1.0, 2.0);
        vec3_compare!(v.neg(), [0.0, -1.0, -2.0]);
    }

    #[test]
    fn vec3_scalar_mul() {
        let scalar = 3.0;
        let v = Vec3::new(0.0, 1.0, 2.0);
        let result1 = v * scalar;
        let result2 = scalar * v;
        assert_eq!(result1, result2);
        vec3_compare!(result1, [0.0, 3.0, 6.0]);
    }

    #[test]
    fn vec3_scalar_div() {
        let scalar = 3.0;
        let v = Vec3::new(0.0, 3.0, 6.0);
        let result1 = v / scalar;
        vec3_compare!(result1, [0.0, 1.0, 2.0]);
    }

    #[test]
    fn vec3_len() {
        let v = Vec3::new(0.0, 3.0, 6.0);
        assert_eq!(v.norm(), (3.0_f32.powi(2) + 6.0_f32.powi(2)).sqrt());
    }

    #[test]
    fn vec3_norm() {
        let v = Vec3::new(4.0, 0.0, 0.0);
        let normalized = v.normalize();
        assert_eq!(normalized.norm(), 1.0);
        assert_eq!(v.normalize(), Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn vec3_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let result1 = v1.cross(&v2);
        let result2 = v2.cross(&v1);
        assert_eq!(result1, result2.neg());
        vec3_compare!(result1, [-1.0, 2.0, -1.0]);
    }

    #[test]
    fn translation_point() {
        let translation = translation!(5.0, -3.0, 2.0);
        let point = point!(-3.0, 4.0, 5.0);
        let result = translation * point;
        vec3_compare!(result, [2.0, 1.0, 7.0]);
    }

    #[test]
    fn translation_vector() {
        let translation = translation!(5.0, 6.0, 7.0);
        let vec = vector!(1.0, 2.0, 3.0);
        let result = translation * vec;
        assert_eq!(result, vec);
    }

    #[test]
    fn scaling_point() {
        let x = scaling!(1.0, 2.0, 3.0);
        let point = point!(1.0, 2.0, 3.0);
        let scaled = x * point;
        assert_eq!(scaled, point!(1.0,4.0,9.0));
    }

    #[test]
    fn scaling_vec3() {
        let scaling = scaling!(1.0, 3.0, 2.0);
        let vec = Vec4::new(1.0, 2.0, 3.0, 0.0);
        let scaled = scaling * vec;

    }

}
