use std::collections::HashMap;
use std::cell::{Cell, RefCell};

use crate::math::*;
//Internal mutability?
pub struct SphereManager {
    next_id: Cell<i32>,
    map: RefCell<HashMap<i32, Sphere>>
}
impl SphereManager {
    pub fn new() -> Self {
        SphereManager{ next_id: Cell::new(0), map: RefCell::new(HashMap::new())}
    }
    pub fn create_sphere(&self) -> i32 { //There will be parameters passed for sphere properties.
        self.map.borrow_mut().insert(self.next_id.get(), Sphere{});
        let sphere_id = self.next_id.get();
        self.next_id.set(sphere_id + 1);
        sphere_id
    }

    pub fn get_sphere(&self, id: i32) -> Option<Sphere> {
        self.map.borrow().get(&id).map( |s| *s) // Sphere copy is here
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
        Ray{ origin, direction }
    }

    pub fn position(&self, t: f32) -> Point4 {
        self.origin + self.direction * t
    }
}



mod test {
    use super::*;
    #[test]
    fn ray_position() {
        let ray = Ray::new(point!(2.0,3.0,4.0), vector!(1.0,0.0,0.0));
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
}

