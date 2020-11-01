use crate::math::*;
use crate::objects::{Ray, Sphere};
use std::cmp::Ordering;

//Scruct representing collision of ray and an object ( Sphere for now )
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f32,
    pub obj: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a Sphere) -> Intersection<'a> {
        if !t.is_finite() {
            panic!("Bad float!");
        }
        Intersection { t, obj: object }
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}
impl<'a> Eq for Intersection<'a> {}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.partial_cmp(&other.t).unwrap()
    }
}

//TODO: Figure out wether we need to group intersections object-wide.
type Intersections<'a> = Vec<Intersection<'a>>;

trait IntersectionInserter<'a> {
    fn add(&mut self, intersection: Intersection<'a>);
}

impl<'a> IntersectionInserter<'a> for Intersections<'a> {
    fn add(&mut self, intersection: Intersection<'a>) {
        let result = self.binary_search(&intersection);
        match result {
            Ok(place) => self.insert(place, intersection),
            Err(place) => self.insert(place, intersection),
        }
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::objects::SphereManager;
    use std::borrow::BorrowMut;

    #[test]
    fn new_intersection() {
        let mut sm = SphereManager::new();
        let (_, sphere) = sm.create_sphere();
        let inter = Intersection::new(3.5, sphere);
        assert_eq!(sphere, inter.obj);
    }

    #[test]
    fn intersection_comparison() {
        let mut sm = SphereManager::new();
        let (_, sphere) = sm.create_sphere();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(-5.0, &sphere);
        assert!(i1 > i2);
    }
    #[test]
    fn intersection_order() {
        let mut sm = SphereManager::new();
        let (_, sphere) = sm.create_sphere();
        let i0 = Intersection::new(5.0, sphere);
        let i1 = Intersection::new(7.0, sphere);
        let i2 = Intersection::new(-3.5, sphere);
        let i3 = Intersection::new(2.0, sphere);

        let mut intersections = Intersections::new();
        intersections.add(i1);
        intersections.add(i0);
        intersections.add(i2);
        intersections.add(i3);

        assert_eq!(intersections.len(), 4 as usize);
        assert_eq!(intersections[0], i2);
        assert_eq!(intersections[1], i3);
        assert_eq!(intersections[2], i0);
        assert_eq!(intersections[3], i1);
    }
}
