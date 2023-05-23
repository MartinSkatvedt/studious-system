use crate::{atoms::Material, scene::SceneNode, sphere::Sphere};

pub struct Planet {
    mass: f64,
    position: glm::Vec3,
    velocity: glm::Vec3,
    radius: f64,
    sphere: Sphere,
}

impl Planet {
    pub fn new(
        mass: f64,
        position: glm::Vec3,
        velocity: glm::Vec3,
        radius: f64,
        material: Material,
    ) -> Planet {
        Planet {
            mass,
            position,
            velocity,
            radius,
            sphere: Sphere::new(5, material),
        }
    }

    pub fn get_sphere(&self) -> &Sphere {
        &self.sphere
    }

    pub fn generate_scene_node(&self, vao_id: u32, shader_id: u32) -> SceneNode {
        SceneNode {
            vao_id: vao_id,
            index_count: self.sphere.shape.index_count,
            position: self.position,
            reference_point: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            shader_program: shader_id,
        }
    }
}
