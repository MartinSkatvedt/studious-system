use crate::material::Material;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub material: Material,
}

#[derive(Copy, Clone)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Triangle {
    pub fn new(a: usize, b: usize, c: usize) -> Triangle {
        Triangle { a, b, c }
    }
}
