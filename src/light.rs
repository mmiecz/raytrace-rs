use crate::material::Material;
use crate::math::*;

pub fn reflection(ray: &Vec4, normal: &Vec4) -> Vec4 {
    ray - normal * 2.0 * ray.dot(normal)
}

#[derive(Debug, Copy, Clone)]
pub struct PointLight {
    pub position: Point4,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point4, intensity: Color) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
}

pub fn lighting(
    material: &Material,
    light_source: &PointLight,
    point: &Point4,
    eyev: &Vec4,
    normalv: &Vec4,
) -> Color {
    let effecticve_color = material.color * light_source.intensity;
    let lightv = (light_source.position - point).normalize();

    let ambient = effecticve_color * material.ambient;

    let diffuse;
    let mut specular = Color::new(0.0, 0.0, 0.0);

    let light_dot_normal = lightv.dot(normalv);
    if light_dot_normal < 0.0 {
        diffuse = Color::new(0.0, 0.0, 0.0);
        specular = Color::new(0.0, 0.0, 0.0);
    } else {
        diffuse = effecticve_color * material.diffuse * light_dot_normal;
        let reflectv = reflection(&-lightv, &normalv);
        let reflect_dot_eye = reflectv.dot(&eyev);
        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light_source.intensity * material.specular * factor;
        }
    }
    ambient + diffuse + specular
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reflect_45_deg() {
        let v = vector!(1.0, -1.0, 0.0);
        let n = vector!(0.0, 1.0, 0.0);
        let r = reflection(&v, &n);
        matrix_eq!(r, vector!(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_slanted() {
        let v = vector!(0.0, -1.0, 0.0);
        let sq = 2.0f32.sqrt() / 2.0;
        let n = vector!(sq, sq, 0.0);
        let r = reflection(&v, &n);
        matrix_eq!(r, vector!(1.0, 0.0, 0.0));
    }

    #[test]
    fn lighting_light_eye_surface() {
        let m = Material::default();
        let position = point!(0.0, 0.0, 0.0);

        let eyev = vector!(0.0, 0.0, -1.0);
        let normalv = vector!(0.0, 0.0, -1.0);
        let light = PointLight::new(point!(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_light_eye_45offset_surface() {
        let m = Material::default();
        let position = point!(0.0, 0.0, 0.0);

        let sq = 2.0f32.sqrt() / 2.0;
        let eyev = vector!(0.0, sq, -sq);
        let normalv = vector!(0.0, 0.0, -1.0);
        let light = PointLight::new(point!(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_eye_light_45offset_surface() {
        let m = Material::default();
        let position = point!(0.0, 0.0, 0.0);

        let eyev = vector!(0.0, 0.0, -1.0);
        let normalv = vector!(0.0, 0.0, -1.0);
        let light = PointLight::new(point!(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_eye_light_oposite_surface() {
        let m = Material::default();
        let position = point!(0.0, 0.0, 0.0);

        let sq = 2.0f32.sqrt() / 2.0;
        let eyev = vector!(0.0, -sq, -sq);
        let normalv = vector!(0.0, 0.0, -1.0);
        let light = PointLight::new(point!(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_eye_surface_light() {
        let m = Material::default();
        let position = point!(0.0, 0.0, 0.0);

        let eyev = vector!(0.0, 0.0, -1.0);
        let normalv = vector!(0.0, 0.0, -1.0);
        let light = PointLight::new(point!(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
