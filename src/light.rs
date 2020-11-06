use crate::math::*;

pub fn reflection(ray: &Vec4, normal: &Vec4) -> Vec4 {
    ray - normal * 2.0 * ray.dot(normal)
}

#[derive(Debug, Copy, Clone)]
struct PointLight {
    position: Vec4,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Vec4, intensity: Color) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
}

mod phong {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reflect_45_deg() {
        let v = vector!(1.0, -1.0, 0.0);
        let n = vector!(0.0, 1.0, 0.0);
        let r = reflection(&v, &n);
        matrix_eq!(r, vector!(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_slanted() {
        let v = vector!(0.0, -1.0, 0.0);
        let sq = 2.0f32.sqrt() / 2.0;
        let n = vector!(sq, sq, 0.0);
        let r = reflection(&v, &n);
        matrix_eq!(r, vector!(1.0, 0.0, 0.0));
    }
}
