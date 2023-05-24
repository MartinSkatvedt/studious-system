use crate::scenenode::SceneNode;
use crate::sphere::Sphere;
use crate::Material;

pub struct Planet {
    pub mass: f64,
    pub position: glm::Vec3,
    pub velocity: glm::Vec3,
    pub radius: f64,
    sphere_object: Sphere,
}

impl Planet {
    pub fn new(
        mass: f64,
        position: glm::Vec3,
        velocity: glm::Vec3,
        radius: f64,
        material: Material,
        detail: u32,
    ) -> Planet {
        Planet {
            mass,
            position,
            velocity,
            radius,
            sphere_object: Sphere::new(detail, material),
        }
    }
    pub fn get_sphere(&mut self) -> &mut Sphere {
        &mut self.sphere_object
    }

    pub fn generate_scene_node(&self, shader_id: u32) -> SceneNode {
        SceneNode {
            vao_id: unsafe { self.sphere_object.mesh.create_vao() },
            index_count: self.sphere_object.mesh.index_count,
            position: self.position,
            reference_point: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            shader_program: shader_id,
        }
    }
}
