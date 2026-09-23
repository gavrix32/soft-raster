use crate::math::Vec3;

pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
}

impl PointLight {
    pub fn new(position: Vec3, color: Vec3) -> Self {
        Self { position, color }
    }
}

pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec3,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, color: Vec3) -> Self {
        Self { direction, color }
    }
}

pub trait LightSource {
    fn add_to_shader(self, shader: &mut Shader);
}

impl LightSource for PointLight {
    fn add_to_shader(self, shader: &mut Shader) {
        shader.point_lights.push(self)
    }
}

impl LightSource for DirectionalLight {
    fn add_to_shader(self, shader: &mut Shader) {
        shader.directional_lights.push(self)
    }
}

pub struct Shader {
    pub ambient_color: Vec3,
    pub point_lights: Vec<PointLight>,
    pub directional_lights: Vec<DirectionalLight>,
}

impl Shader {
    pub fn new(ambient_color: Vec3) -> Shader {
        Self {
            ambient_color,
            point_lights: Vec::new(),
            directional_lights: Vec::new(),
        }
    }

    pub fn add_light(&mut self, light: impl LightSource) {
        light.add_to_shader(self)
    }

    pub fn shade(&self, base_color: Vec3, position: Vec3, normal: Vec3) -> Vec3 {
        let n = normal.normalize();
        let mut result = base_color * self.ambient_color;
        for light in &self.point_lights {
            let light_dir = (light.position - position).normalize();
            let n_dot_l = n.dot(light_dir).max(0.0);
            result += base_color * light.color * n_dot_l;
        }
        for light in &self.directional_lights {
            let n_dot_l = n.dot(light.direction).max(0.0);
            result += base_color * light.color * n_dot_l;
        }
        result
    }
}
