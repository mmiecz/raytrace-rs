use crate::math::*;
use crate::objects::{normal_at, Ray, Sphere};
use std::cmp::Ordering;

//Struct representing collision of ray and an object ( Sphere for now )
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

//TODO: Figure out whether we need to group intersections object-wide.
pub type Intersections<'a> = Vec<Intersection<'a>>;

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

#[derive(Debug, Copy, Clone)]
pub struct Precomputation<'a> {
    pub t: f32,
    pub obj: &'a Sphere,
    pub point: Point4,
    pub eyev: Vec4,
    pub normalv: Vec4,
    pub inside: bool,
}

impl<'a> Precomputation<'a> {
    pub fn compute(intersection: &Intersection<'a>, ray: &Ray) -> Precomputation<'a> {
        let pos = ray.position(intersection.t);
        let normalv = normal_at(intersection.obj, &pos);
        let eyev = -ray.direction;
        let inside = normalv.dot(&eyev) < 0.0;
        //Are we inside the object?
        Precomputation {
            t: intersection.t,
            obj: intersection.obj,
            point: pos,
            eyev,
            normalv: {
                if inside {
                    -normalv
                } else {
                    normalv
                }
            },
            inside,
        }
    }
}

//Calculate if ray is intersecting with a sphere
//Returns Some(points of intersection) where there is a hit, or None otherwise
pub fn intersect<'a>(ray: &Ray, sphere: &'a Sphere) -> Option<Intersections<'a>> {
    //Sphere center to the origin.
    let transformation = sphere
        .get_transformation()
        .clone_owned()
        .try_inverse() // This will panic!
        .expect("Unable to inverse transformation matrix for intersection!");
    let ray_transformed = ray.transform(&transformation);
    let sphere_to_ray = ray_transformed.origin - point!(0.0, 0.0, 0.0);

    let a = ray_transformed.direction.dot(&ray_transformed.direction);
    let b = 2.0 * ray_transformed.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        None // NO HIT
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let denom = 2.0 * a;
        let t1 = (-b - sqrt_discriminant) / denom;
        let t2 = (-b + sqrt_discriminant) / denom;
        let mut ts = vec![t1, t2];
        ts.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut intersections = Intersections::new();
        for t in ts.iter() {
            intersections.add(Intersection::new(*t, &sphere));
        }
        Some(intersections)
    }
}

//Return first visible hit from intersections hits.
pub fn hit<'a>(intersections: &'a Intersections) -> Option<&'a Intersection<'a>> {
    for intersect in intersections.into_iter() {
        if intersect.t > 0.0 {
            return Some(&intersect);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::objects::SphereBuilder;
    use std::path::PrefixComponent;

    #[test]
    fn new_intersection() {
        let sphere = SphereBuilder::new().create();
        let inter = Intersection::new(3.5, &sphere);
        assert_eq!(&sphere, inter.obj);
    }

    #[test]
    fn intersection_comparison() {
        let sphere = SphereBuilder::new().create();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(-5.0, &sphere);
        assert!(i1 > i2);
    }
    #[test]
    fn intersection_order() {
        let sphere = SphereBuilder::new().create();
        let i0 = Intersection::new(5.0, &sphere);
        let i1 = Intersection::new(7.0, &sphere);
        let i2 = Intersection::new(-3.5, &sphere);
        let i3 = Intersection::new(2.0, &sphere);

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

    #[test]
    fn test_ray_hit() {
        let sphere = SphereBuilder::new().create();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(2.0, &sphere);
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);
        assert_eq!(Some(&i1), hit(&inters));
    }

    #[test]
    fn test_ray_hit_with_negatives() {
        let sphere = SphereBuilder::new().create();
        let i1 = Intersection::new(-1.0, &sphere);
        let i2 = Intersection::new(0.1, &sphere);
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);
        assert_eq!(Some(&i2), hit(&inters));
    }

    #[test]
    fn test_no_ray_hits() {
        let sphere = SphereBuilder::new().create();
        let i1 = Intersection::new(-1.0, &sphere);
        let i2 = Intersection::new(-0.1, &sphere);
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);
        assert_eq!(None, hit(&inters));
    }

    #[test]
    fn test_nearest_hit() {
        let sphere = SphereBuilder::new().create();
        let i1 = Intersection::new(5.0, &sphere);
        let i2 = Intersection::new(7.0, &sphere);
        let i3 = Intersection::new(-3.0, &sphere);
        let i4 = Intersection::new(2.0, &sphere);
        let mut inters = Intersections::new();
        inters.add(i1);
        inters.add(i2);
        inters.add(i3);
        inters.add(i4);
        assert_eq!(Some(&i4), hit(&inters));
    }

    #[test]
    fn test_transformed_sphere_ray_hit() {
        let mut sphere = SphereBuilder::new().create();
        let r = Ray::new(point!(0.0, 0.0, -5.0), vector!(0.0, 0.0, 1.0));
        sphere.transform(&translation!(5.0, 0.0, 0.0));
        assert_eq!(None, intersect(&r, &sphere));
    }
    #[test]
    fn test_scaled_sphere_ray_hit() {
        let mut sphere = SphereBuilder::new().create();
        let r = Ray::new(point!(0.0, 0.0, -5.0), vector!(0.0, 0.0, 1.0));
        sphere.transform(&scaling!(2.0, 2.0, 2.0));
        let intersects = intersect(&r, &sphere);
        assert_eq!(intersects.as_ref().map(|res| res.is_empty()), Some(false));

        let expected = vec![
            Intersection::new(3.0, &sphere),
            Intersection::new(7.0, &sphere),
        ];
        assert!(intersects
            .unwrap()
            .iter()
            .zip(&expected)
            .all(|(a, b)| a == b));
    }

    #[test]
    fn test_outside_intesection_precomputation() {
        let ray = Ray::new(point!(0.0, 0.0, -5.0), vector!(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);
        let comps = Precomputation::compute(&i, &ray);
        assert_eq!(comps.obj, i.obj);
        assert_eq!(comps.t, i.t);
        matrix_eq!(comps.point, point!(0.0, 0.0, -1.0));
        matrix_eq!(comps.eyev, vector!(0.0, 0.0, -1.0));
        matrix_eq!(comps.normalv, vector!(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn test_inside_intesection_precomputation() {
        let ray = Ray::new(point!(0.0, 0.0, 0.0), vector!(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(1.0, &shape);
        let comps = Precomputation::compute(&i, &ray);
        assert_eq!(comps.obj, i.obj);
        assert_eq!(comps.t, i.t);
        matrix_eq!(comps.point, point!(0.0, 0.0, 1.0));
        matrix_eq!(comps.eyev, vector!(0.0, 0.0, -1.0));
        matrix_eq!(comps.normalv, vector!(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
    }
}
