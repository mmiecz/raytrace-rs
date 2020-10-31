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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
pub fn intersect<'a>(ray: &Ray, sphere: &'a Sphere) -> Option<Intersections<'a>> {
    //Sphere center to the origin.
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
        let mut ts = vec![t1, t2];
        ts.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut intersections = Intersections::new();
        for t in ts.iter() {
            intersections.push(Intersection::new(*t, &sphere));
        }
        Some(intersections)
    }
}
//Scruct representing collision of ray and an object ( Sphere for now )
#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f32,
    pub obj: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a Sphere) -> Intersection<'a> {
        Intersection { t, obj: object }
    }
}
type Intersections<'a> = Vec<Intersection<'a>>;

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
        let sphere = sm.get_sphere(s).unwrap();
        let intersects = intersect(&ray, &sphere);
        assert_eq!(intersects.as_ref().map(|res| res.is_empty()), Some(false));

        let expected = vec![
            Intersection::new(4.0, &sphere),
            Intersection::new(6.0, &sphere),
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
        let sm = SphereManager::new();
        let s = sm.create_sphere();
        let sphere = sm.get_sphere(s).unwrap();
        let intersects = intersect(&ray, &sphere);
        assert_eq!(intersects.as_ref().map(|res| res.is_empty()), Some(false));

        let expected = vec![
            Intersection::new(5.0, &sphere),
            Intersection::new(5.0, &sphere),
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
        let sm = SphereManager::new();
        let s = sm.create_sphere();
        let sphere = sm.get_sphere(s).unwrap();
        let inter = intersect(&ray, &sphere);
        assert_eq!(inter.is_none(), true);
    }

    #[test]
    fn ray_origin_inside_sphere() {
        let ray = Ray::new(point!(0.0, 0.0, 0.0), vector!(0.0, 0.0, 1.0));
        let sm = SphereManager::new();
        let s = sm.create_sphere();
        let sphere = sm.get_sphere(s).unwrap();
        let intersects = intersect(&ray, &sphere);
        assert_eq!(intersects.as_ref().map(|res| res.is_empty()), Some(false));

        let expected = vec![
            Intersection::new(-1.0, &sphere),
            Intersection::new(1.0, &sphere),
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
        let sm = SphereManager::new();
        let s = sm.create_sphere();
        let sphere = sm.get_sphere(s).unwrap();
        let intersects = intersect(&ray, &sphere);
        assert_eq!(intersects.as_ref().map(|res| res.is_empty()), Some(false));

        let expected = vec![
            Intersection::new(-6.0, &sphere),
            Intersection::new(-4.0, &sphere),
        ];
        assert!(intersects
            .unwrap()
            .iter()
            .zip(&expected)
            .all(|(a, b)| a == b));
    }

    #[test]
    fn new_intersection() {
        let sm = SphereManager::new();
        let s = sm.create_sphere();
        let sphere = sm.get_sphere(s).unwrap();
        let inter = Intersection::new(3.5, &sphere);
        assert_eq!(&sphere, inter.obj);
    }
    #[test]
    fn add_new_intersection_to_intersections() {
        let sm = SphereManager::new();
        let s = sm.create_sphere();
        let sphere = sm.get_sphere(s).unwrap();
        let intersection = Intersection::new(3.5, &sphere);

        let mut intersections = Intersections::new();
        intersections.push(intersection);
        assert_eq!(intersections.len(), 1 as usize);
    }
}
