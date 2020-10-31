use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use crate::math::*;
//Internal mutability?
pub struct SphereManager {
    next_id: Cell<i32>,
    map: RefCell<HashMap<i32, Sphere>>, // Should Sphere be Rc?
}

impl SphereManager {
    pub fn new() -> Self {
        SphereManager {
            next_id: Cell::new(0),
            map: RefCell::new(HashMap::new()),
        }
    }
    pub fn create_sphere(&self) -> i32 {
        //There will be parameters passed for sphere properties.
        self.map.borrow_mut().insert(self.next_id.get(), Sphere {});
        let sphere_id = self.next_id.get();
        self.next_id.set(sphere_id + 1);
        sphere_id
    }

    pub fn get_sphere(&self, id: i32) -> Option<Sphere> {
        self.map.borrow().get(&id).map(|s| *s) // Sphere copy is here
    }
}

//Empty sphere that is placed in the center of the screen and has a radius of 1.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sphere;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Point4,
    direction: Vec4,
}

impl Ray {
    pub fn new(origin: Point4, direction: Vec4) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f32) -> Point4 {
        self.origin + self.direction * t
    }
}
//Calculate if ray is intersecting with a sphere
//TODO: For now, sphere is placed in 0,0,0
//Returns Some(points of intersection) where there is a hit, or None otherwise
pub fn intersect(ray: &Ray, sphere: &Sphere) -> Option<Vec<f32>> {
    //Sphere cetnter to the origin.
    let sphere_to_ray = ray.origin - point!(0.0, 0.0, 0.0); // point here is fixed for now in 0,0,0
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * ray.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        None
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let denom = 2.0 * a;
        let t1 = (-b - sqrt_discriminant) / denom;
        let t2 = (-b + sqrt_discriminant) / denom;
        let mut res = vec![t1, t2];
        res.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Some(res)
    }
}

mod test {
    use super::*;
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
        let manager = SphereManager::new();
        let id = manager.create_sphere();
        let id2 = manager.create_sphere();
        assert!(id != id2);
        let sphere = manager.get_sphere(id);
        assert!(sphere.is_some());
        let nonexistent = manager.get_sphere(100);
        assert!(nonexistent.is_none());
    }

    #[test]
    fn ray_sphere_intersection_two_points() {
        let ray = Ray::new(point!(0.0, 0.0, -5.0), vector!(0.0, 0.0, 1.0));
        let sm = SphereManager::new();
        let s = sm.create_sphere();
        let intersecs = intersect(&ray, &sm.get_sphere(s).unwrap());
        assert_eq!(intersecs.as_ref().map(|res| res.is_empty()), Some(false));
        assert_eq!(
            intersecs.as_ref().map(|res| (res[0], res[1])),
            Some((4.0, 6.0))
        );
    }

    #[test]
    fn ray_sphere_intersection_tangent() {
        let ray = Ray::new(point!(0.0, 1.0, -5.0), vector!(0.0, 0.0, 1.0));
        let sm = SphereManager::new();
        let sphere = sm.create_sphere();
        let inter = intersect(&ray, &sm.get_sphere(sphere).unwrap());
        assert_eq!(inter.as_ref().map(|res| res.is_empty()), Some(false));
        assert_eq!(inter.as_ref().map(|res| (res[0], res[1])), Some((5.0, 5.0)));
    }

    #[test]
    fn ray_sphere_intersection_none() {
        let ray = Ray::new(point!(0.0, 2.0, -5.0), vector!(0.0, 0.0, 1.0));
        let sm = SphereManager::new();
        let s = sm.create_sphere();
        let i = intersect(&ray, &sm.get_sphere(s).unwrap());
        assert_eq!(i.is_none(), true);
    }

    #[test]
    fn ray_origin_inside_sphere() {
        let ray = Ray::new(point!(0.0, 0.0, 0.0), vector!(0.0, 0.0, 1.0));
        let sm = SphereManager::new();
        let s = sm.create_sphere();
        let inter = intersect(&ray, &sm.get_sphere(s).unwrap());
        assert_eq!(inter.as_ref().map(|res| res.is_empty()), Some(false));
        assert_eq!(
            inter.as_ref().map(|res| (res[0], res[1])),
            Some((-1.0, 1.0))
        );
    }

    #[test]
    fn ray_origin_behind_sphere() {
        let ray = Ray::new(point!(0.0, 0.0, 5.0), vector!(0.0, 0.0, 1.0));
        let sm = SphereManager::new();
        let s = sm.create_sphere();
        let inter = intersect(&ray, &sm.get_sphere(s).unwrap());
        assert_eq!(inter.as_ref().map(|res| res.is_empty()), Some(false));
        assert_eq!(
            inter.as_ref().map(|res| (res[0], res[1])),
            Some((-6.0, -4.0))
        );
    }
}
