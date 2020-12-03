use crate::intersection::Precomputation;
use crate::material::Material;
use crate::math::*;
use crate::world::World;

fn reflection(ray: &Vec4, normal: &Vec4) -> Vec4 {
    ray - normal * 2.0 * ray.dot(normal)
}

pub trait LightSource {
    fn illuminate(&self, material: &Material, point: &Point4, eyev: &Vec4, normalv: &Vec4)
        -> Color;
}

impl LightSource for PointLight {
    fn illuminate(
        &self,
        material: &Material,
        point: &Point4,
        eyev: &Vec4,
        normalv: &Vec4,
    ) -> Color {
        let effecticve_color = material.color * self.intensity;
        let lightv = (self.position - point).normalize();

        let ambient = effecticve_color * material.ambient;

        let mut diffuse = Color::new(0.0, 0.0, 0.0);
        let mut specular = Color::new(0.0, 0.0, 0.0);

        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0.0 {
            diffuse = Color::new(0.0, 0.0, 0.0);
            specular = Color::new(0.0, 0.0, 0.0);
        } else {
            diffuse = effecticve_color * material.diffuse * light_dot_normal;
            let reflectv = reflection(&-lightv, &normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);
            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = self.intensity * material.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

pub fn shade_hit(world: &World, precomps: &Precomputation) -> Color {
    let light = world.light_source(); // Single light source supported for now.
    light.illuminate(
        &precomps.obj.material,
        &precomps.point,
        &precomps.eyev,
        &precomps.normalv,
    )
}

#[derive(Debug, Copy, Clone)]
pub struct PointLight {
    pub position: Point4,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point4, intensity: Color) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::intersection::{Intersection, Precomputation};
    use crate::objects::Ray;

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

    #[test]
    fn lighting_light_eye_surface() {
        let m = Material::default();
        let position = point!(0.0, 0.0, 0.0);

        let eyev = vector!(0.0, 0.0, -1.0);
        let normalv = vector!(0.0, 0.0, -1.0);
        let light = PointLight::new(point!(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = light.illuminate(&m, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_light_eye_45offset_surface() {
        let m = Material::default();
        let position = point!(0.0, 0.0, 0.0);

        let sq = 2.0f32.sqrt() / 2.0;
        let eyev = vector!(0.0, sq, -sq);
        let normalv = vector!(0.0, 0.0, -1.0);
        let light = PointLight::new(point!(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = light.illuminate(&m, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_eye_light_45offset_surface() {
        let m = Material::default();
        let position = point!(0.0, 0.0, 0.0);

        let eyev = vector!(0.0, 0.0, -1.0);
        let normalv = vector!(0.0, 0.0, -1.0);
        let light = PointLight::new(point!(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = light.illuminate(&m, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_eye_light_oposite_surface() {
        let m = Material::default();
        let position = point!(0.0, 0.0, 0.0);

        let sq = 2.0f32.sqrt() / 2.0;
        let eyev = vector!(0.0, -sq, -sq);
        let normalv = vector!(0.0, 0.0, -1.0);
        let light = PointLight::new(point!(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = light.illuminate(&m, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_eye_surface_light() {
        let m = Material::default();
        let position = point!(0.0, 0.0, 0.0);

        let eyev = vector!(0.0, 0.0, -1.0);
        let normalv = vector!(0.0, 0.0, -1.0);
        let light = PointLight::new(point!(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = light.illuminate(&m, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_shade_hit() {
        let w = World::default();
        let r = Ray::new(point!(0.0, 0.0, -5.0), vector!(0.0, 0.0, 1.0));
        let shape = w
            .shapes_iter()
            .next()
            .expect("Expected some shaped in the world!");
        let i = Intersection::new(4.0, shape);
        let precomps = Precomputation::compute(&i, &r);
        let c = shade_hit(&w, &precomps);
        assert!((c.r() - 0.38066).abs() < 0.00001);
        assert!((c.g() - 0.47583).abs() < 0.00001);
        assert!((c.b() - 0.2855).abs() < 0.00001);
    }
}
