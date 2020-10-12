use nalgebra as na;
use std::ops::Neg;

type Vec3 = na::Vector3<f32>;
type Mat3 = na::Matrix3<f32>;
type Point = na::Point3<f32>;

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

    fn vec3_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let result1 = v1.cross(&v2);
        let result2 = v2.cross(&v1);
        assert_eq!(result1, result2.neg());
        vec3_compare!(result1, [-1.0, 2.0, -1.0]);
    }
}
