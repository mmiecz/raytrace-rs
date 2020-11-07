use crate::math::*;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

//TODO: Implement builder for better params handling.
impl Material {
    pub fn new(
        color: Color,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Material {
        unimplemented!()
    }

    pub fn default_with_color(color: Color) -> Material {
        Material {
            color,
            ..Default::default()
        }
    }
}
