use crate::math::*;
use std::collections::HashMap;

pub struct SphereManager {
    next_id: i32,
    map: HashMap<i32, Sphere>, // Should Sphere be Rc?
}

impl SphereManager {
    pub fn new() -> Self {
        SphereManager {
            next_id: 0,
            map: HashMap::new(),
        }
    }
    pub fn create_sphere(&mut self) -> (i32, &Sphere) {
        //There will be parameters passed for sphere properties.
        self.map.insert(self.next_id, Sphere {});
        let sphere_id = self.next_id;
        self.next_id += 1;
        (sphere_id, self.map.get(&sphere_id).unwrap())
    }
}

//Empty sphere that is placed in the center of the screen and has a radius of 1.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Sphere;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point4,
    pub direction: Vec4,
}

impl Ray {
    pub fn new(origin: Point4, direction: Vec4) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f32) -> Point4 {
        self.origin + self.direction * t
    }

    pub fn transform(&self, transform_matrix: &Mat4) -> Ray {
        Ray {
            origin: transform_matrix * self.origin,
            direction: transform_matrix * self.direction,
        }
    }
}

mod test {
    use super::*;
    use crate::intersection::*;
    #[test]
    fn ray_position() {
        let ray = Ray::new(point!(2.0, 3.0, 4.0), vector!(1.0, 0.0, 0.0));
        let p1 = ray.position(0.0);
        assert_eq!(point!(2.0, 3.0, 4.0), p1);
        let p2 = ray.position(1.0);
        assert_eq!(point!(3.0, 3.0, 4.0), p2);
        let p3 = ray.position(-1.0);
        assert_eq!(point!(1.0, 3.0, 4.0), p3);
    }

    #[test]
    fn sphere_creation() {
        let mut manager = SphereManager::new();
        let (id, _) = manager.create_sphere();
        let (id2, _) = manager.create_sphere();
        assert_ne!(id, id2);
    }

    #[test]
    fn ray_sphere_intersection_two_points() {
        let ray = Ray::new(point!(0.0, 0.0, -5.0), vector!(0.0, 0.0, 1.0));
        let mut sm = SphereManager::new();
        let (_, sphere) = sm.create_sphere();
        let intersects = intersect(&ray, sphere);
        assert_eq!(intersects.as_ref().map(|res| res.is_empty()), Some(false));

        let expected = vec![
            Intersection::new(4.0, sphere),
            Intersection::new(6.0, sphere),
        ];
        assert!(intersects
            .unwrap()
            .iter()
            .zip(&expected)
            .all(|(a, b)| a == b));
    }

    #[test]
    fn ray_sphere_intersection_tangent() {
        let ray = Ray::new(point!(0.0, 1.0, -5.0), vector!(0.0, 0.0, 1.0));
        let mut sm = SphereManager::new();
        let (_, sphere) = sm.create_sphere();
        let intersects = intersect(&ray, sphere);
        assert_eq!(intersects.as_ref().map(|res| res.is_empty()), Some(false));

        let expected = vec![
            Intersection::new(5.0, sphere),
            Intersection::new(5.0, sphere),
        ];
        assert!(intersects
            .unwrap()
            .iter()
            .zip(&expected)
            .all(|(a, b)| a == b));
    }

    #[test]
    fn ray_sphere_intersection_none() {
        let ray = Ray::new(point!(0.0, 2.0, -5.0), vector!(0.0, 0.0, 1.0));
        let mut sm = SphereManager::new();
        let (_, sphere) = sm.create_sphere();
        let inter = intersect(&ray, sphere);
        assert_eq!(inter.is_none(), true);
    }

    #[test]
    fn ray_origin_inside_sphere() {
        let ray = Ray::new(point!(0.0, 0.0, 0.0), vector!(0.0, 0.0, 1.0));
        let mut sm = SphereManager::new();
        let (_, sphere) = sm.create_sphere();
        let intersects = intersect(&ray, sphere);
        assert_eq!(intersects.as_ref().map(|res| res.is_empty()), Some(false));

        let expected = vec![
            Intersection::new(-1.0, sphere),
            Intersection::new(1.0, sphere),
        ];
        assert!(intersects
            .unwrap()
            .iter()
            .zip(&expected)
            .all(|(a, b)| a == b));
    }

    #[test]
    fn ray_origin_behind_sphere() {
        let ray = Ray::new(point!(0.0, 0.0, 5.0), vector!(0.0, 0.0, 1.0));
        let mut sm = SphereManager::new();
        let (_, sphere) = sm.create_sphere();
        let intersects = intersect(&ray, sphere);
        assert_eq!(intersects.as_ref().map(|res| res.is_empty()), Some(false));

        let expected = vec![
            Intersection::new(-6.0, sphere),
            Intersection::new(-4.0, sphere),
        ];
        assert!(intersects
            .unwrap()
            .iter()
            .zip(&expected)
            .all(|(a, b)| a == b));
    }

    #[test]
    fn ray_translation() {
        let r = Ray::new(point!(1.0, 2.0, 3.0), vector!(0.0, 1.0, 0.0));
        let m = translation!(3.0, 4.0, 5.0);
        let result = r.transform(&m);
        assert_eq!(result.origin, point!(4.0, 6.0, 8.0));
        assert_eq!(result.direction, vector!(0.0, 1.0, 0.0));
    }

    #[test]
    fn ray_scaling() {
        let r = Ray::new(point!(1.0, 2.0, 3.0), vector!(0.0, 1.0, 0.0));
        let m = scaling!(2.0, 3.0, 4.0);
        let result = r.transform(&m);
        assert_eq!(result.origin, point!(2.0, 6.0, 12.0));
        assert_eq!(result.direction, vector!(0.0, 3.0, 0.0));
    }
}
