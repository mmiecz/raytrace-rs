use std::ops::{Add, Sub};

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
}
