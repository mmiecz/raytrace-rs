use crate::intersection::*;
use crate::light::{LightSource, PointLight};
use crate::material::Material;
use crate::math::*;
use crate::objects::{Ray, Sphere};
use crate::SphereBuilder;

struct World {
    objects: Vec<Sphere>, // TODO: Maybe there should be generic Object.
    light_source: Box<dyn LightSource>, // TODO: Maybe there should be generic LightSource.
}

impl Default for World {
    fn default() -> Self {
        let mut sb = SphereBuilder::new();
        let s1 = sb.with_transformation(scaling!(0.5, 0.5, 0.5)).create();
        let s2 = sb
            .with_material(Material::new(
                Color::new(0.8, 1.0, 0.6),
                0.1,
                0.7,
                0.2,
                200.0,
            ))
            .create();
        let objects = vec![s1, s2];
        let light = PointLight::new(point!(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        World {
            objects,
            light_source: Box::new(light),
        }
    }
}

impl World {
    //Find all intersections with all objects in the world
    pub fn ray_intersect(&self, ray: &Ray) -> Intersections {
        let mut result = Vec::new();
        for object in &self.objects {
            if let Some(mut intersections) = intersect(ray, object) {
                result.append(&mut intersections);
            }
        }
        result.sort();
        result
    }
}

#[cfg(test)]
mod test {
    use crate::intersection::Intersection;
    use crate::math::*;
    use crate::objects::Ray;
    use crate::world::World;

    #[test]
    fn ray_world_intersection() {
        let w = World::default();
        let ray = Ray::new(point!(0.0, 0.0, -5.0), vector!(0.0, 0.0, 1.0));
        let result = w.ray_intersect(&ray);
        let expected = vec![4.0, 4.5, 5.5, 6.0];
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].t, expected[0]);
        assert_eq!(result[1].t, expected[1]);
        assert_eq!(result[2].t, expected[2]);
        assert_eq!(result[3].t, expected[3]);
    }
}
