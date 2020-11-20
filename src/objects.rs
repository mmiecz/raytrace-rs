use crate::material::Material;
use crate::math::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

///TODO: Create Object trait and implement for Sphere

pub struct SphereBuilder {
    transformation: Option<Mat4>,
    material: Option<Material>,
}

impl SphereBuilder {
    pub fn new() -> SphereBuilder {
        SphereBuilder {
            transformation: None,
            material: None,
        }
    }

    pub fn with_transformation(&mut self, transformation: Mat4) -> &mut SphereBuilder {
        self.transformation.replace(transformation);
        self
    }

    pub fn with_material(&mut self, material: Material) -> &mut SphereBuilder {
        self.material.replace(material);
        self
    }

    pub fn create(&mut self) -> Sphere {
        static COUNTER: AtomicU32 = AtomicU32::new(1);
        let result = Sphere::new(
            COUNTER.fetch_add(1, Ordering::Relaxed),
            self.transformation.unwrap_or_else(Mat4::identity),
            self.material.unwrap_or_default(),
        );
        self.transformation = None;
        self.material = None;
        result
    }
}

//Empty sphere that is placed in the center of the screen and has a radius of 1.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Sphere {
    id: u32,
    transformation: Mat4,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            id: 0,
            transformation: Mat4::identity(),
            material: Material::default(),
        }
    }
}

impl Sphere {
    pub(self) fn new(id: u32, transformation: Mat4, material: Material) -> Sphere {
        Sphere {
            id,
            transformation,
            material,
        }
    }

    /// This expects homogeneous matrix
    pub fn transform(&mut self, transformation: &Mat4) -> &mut Self {
        self.transformation *= transformation;
        self
    }

    pub fn get_transformation(&self) -> &Mat4 {
        &self.transformation
    }
    pub fn id(&self) -> u32 {
        self.id
    }
}

pub fn normal_at(sphere: &Sphere, world_point: &Point4) -> Vec4 {
    let inversed_sphere_transform = sphere
        .get_transformation()
        .try_inverse()
        .expect("Can't inverse transformation matrix for sphere!");

    let object_point = inversed_sphere_transform * world_point;
    let object_normal = object_point - point!(0.0, 0.0, 0.0);

    let mut world_normal = inversed_sphere_transform.transpose() * object_normal;
    world_normal.w = 0.0;
    world_normal.normalize()
}

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

#[cfg(test)]
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
        let mut sb = SphereBuilder::new();
        let s1 = sb.create();
        let s2 = sb.create();
        assert_ne!(s1.id, s2.id);
    }

    #[test]
    fn ray_sphere_intersection_two_points() {
        let ray = Ray::new(point!(0.0, 0.0, -5.0), vector!(0.0, 0.0, 1.0));
        let sphere = SphereBuilder::new().create();

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
        let sphere = SphereBuilder::new().create();
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
        let sphere = SphereBuilder::new().create();
        let inter = intersect(&ray, &sphere);
        assert_eq!(inter.is_none(), true);
    }

    #[test]
    fn ray_origin_inside_sphere() {
        let ray = Ray::new(point!(0.0, 0.0, 0.0), vector!(0.0, 0.0, 1.0));
        let sphere = SphereBuilder::new().create();
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
        let sphere = SphereBuilder::new().create();
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

    #[test]
    fn sphere_default_transformation_matrix() {
        let sphere = SphereBuilder::new().create();
        assert_eq!(sphere.get_transformation(), &Mat4::identity());
    }

    #[test]
    fn shpere_translate() {
        let t = translation!(2.0, 3.0, 4.0);
        let sphere = SphereBuilder::new().with_transformation(t).create();
        assert_eq!(&t, sphere.get_transformation());
    }

    #[test]
    fn normal_at_sphere() {
        let sphere = SphereBuilder::new().create();
        let point = point!(1.0, 0.0, 0.0);
        let normal = normal_at(&sphere, &point);
        assert_eq!(normal, vector!(1.0, 0.0, 0.0));
        let point = point!(0.0, 1.0, 0.0);
        let normal = normal_at(&sphere, &point);
        assert_eq!(normal, Vec4::new(0.0, 1.0, 0.0, 0.0));
        let sq = 3f32.sqrt() / 3.0;
        let point = point!(sq, sq, sq);
        let normal = normal_at(&sphere, &point);
        matrix_eq!(normal, vector!(sq, sq, sq));
    }

    #[test]
    fn normal_is_normalized() {
        let sphere = SphereBuilder::new().create();
        let sq = 3f32.sqrt() / 3.0;
        let point = point!(sq, sq, sq);
        let normal = normal_at(&sphere, &point);
        matrix_eq!(normal, normal.normalize());
    }

    #[test]
    fn normal_on_translated_sphere() {
        use std::f32::consts::FRAC_1_SQRT_2;
        let mut sphere = SphereBuilder::new().create();
        sphere.transform(&translation!(0.0, 1.0, 0.0));
        let normal = normal_at(&sphere, &point!(0.0, 1.70711, -FRAC_1_SQRT_2));
        matrix_eq!(normal, vector!(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normal_on_scaled_rotated_sphere() {
        let m = scaling!(1.0, 0.5, 1.0) * rotation!(0.0, 0.0, std::f32::consts::PI / 5.0);
        let mut sphere = SphereBuilder::new().with_transformation(m).create();
        let sq = 2.0f32.sqrt() / 2.0;
        let normal = normal_at(&sphere, &point!(0.0, sq, -sq));
        matrix_eq!(normal, vector!(0.0, 0.97014, -0.24254));
    }
}
