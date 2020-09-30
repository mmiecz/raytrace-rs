use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign, DivAssign};

#[derive(Copy, Clone, Debug)]
struct Vec3([f32; 3]);

impl Vec3 {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Vec3{0: [a,b,c] }
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
}

//TODO: Move this to macro
// Op<Vec3> for `Scalar`
// Op<`Scalar`> for Vec3
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            0: [self.0[0] - rhs.0[0],
                self.0[1] - rhs.0[1],
                self.0[2] - rhs.0[2] ]
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

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            0: [self.0[0] + rhs.0[0],
                self.0[1] + rhs.0[1],
                self.0[2] + rhs.0[2] ]
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
            0: [self.0[0] * rhs,
                self.0[1] * rhs,
                self.0[2] * rhs ]
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
    #[test]
    fn test_vec3() {
        let v = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 1.0);
        assert_eq!(v.z(), 2.0);
    }

    #[test]
    fn test_add() {
        let v = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(0.0, 1.0, 2.0);
        let result = v + v2;
        assert_eq!(result.x(), 0.0);
        assert_eq!(result.y(), 2.0);
        assert_eq!(result.z(), 4.0);
    }

    #[test]
    fn test_mul() {
        let v = Vec3::new(0.0, 1.0, 5.0);
        let v = v * 3.0;
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 3.0);
        assert_eq!(v.z(), 15.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec3::new(0.0, 1.0, 5.0);
        v *= 3.0;
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 3.0);
        assert_eq!(v.z(), 15.0);
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec3::new(0.0, 1.0, 5.0);
        v /= 2.0;
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.5);
        assert_eq!(v.z(), 2.5);
    }
}
