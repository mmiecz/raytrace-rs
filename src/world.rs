use crate::intersection::{intersect, Intersection};
use crate::light::{LightSource, PointLight};
use crate::material::Material;
use crate::math::*;
use crate::objects::{Ray, Sphere, SphereBuilder};

pub struct World {
    light_source: Box<dyn LightSource>,
    //Should be Object in future?
    objects: Vec<Sphere>,
}

///Default World has two spheres, and one light source.
impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(point!(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut sb = SphereBuilder::new();
        let s1 = sb
            .with_material(Material::new(
                Color::new(0.8, 1.0, 0.6),
                0.1,
                0.7,
                0.2,
                200.0,
            ))
            .create();
        let s2 = sb.with_transformation(scaling!(0.5, 0.5, 0.5)).create();
        World {
            light_source: Box::new(light),
            objects: vec![s1, s2],
        }
    }
}

impl World {
    pub fn ray_intersect<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        let mut result: Vec<Intersection<'a>> = Vec::new();
        for object in &self.objects {
            if let Some(mut intersections) = intersect(ray, &object) {
                result.append(&mut intersections);
            }
        }

        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_world_creation() {
        let world = World::default();
    }

    fn default_world_ray_intersection() {
        let world = World::default();
        let ray = Ray::new(point!(0.0, 0.0, -5.0), vector!(0.0, 0.0, 1.0));
        let intersections = world.ray_intersect(&ray);
        assert_eq!(intersections.len(), 4);
        assert!((intersections[0].t - 4.0).abs() < std::f32::EPSILON);
        assert!((intersections[0].t - 4.5).abs() < std::f32::EPSILON);
        assert!((intersections[0].t - 5.5).abs() < std::f32::EPSILON);
        assert!((intersections[0].t - 6.0).abs() < std::f32::EPSILON);
    }
}
