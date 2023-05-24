use crate::material::Material;
use crate::mesh::{Mesh, MeshMaterial};
use crate::vertex::{Triangle, Vertex};
use lerp::Lerp;

pub struct Sphere {
    vertices: Vec<Vertex>,
    displaced_vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
    pub mesh: Mesh,
    pub mesh_material: Material,
}

impl Sphere {
    pub fn new(detail: u32, material: Material) -> Sphere {
        let phi = (1.0 + (5.0 as f32).sqrt()) / 2.0;
        let mut regular_isocahedron = Sphere {
            vertices: vec![
                Vertex {
                    position: glm::vec3(-1.0, phi, 0.0),
                    material: material,
                },
                Vertex {
                    position: glm::vec3(1.0, phi, 0.0),
                    material: material,
                },
                Vertex {
                    position: glm::vec3(-1.0, -phi, 0.0),
                    material: material,
                },
                Vertex {
                    position: glm::vec3(1.0, -phi, 0.0),
                    material: material,
                },
                Vertex {
                    position: glm::vec3(0.0, -1.0, phi),
                    material: material,
                },
                Vertex {
                    position: glm::vec3(0.0, 1.0, phi),
                    material: material,
                },
                Vertex {
                    position: glm::vec3(0.0, -1.0, -phi),
                    material: material,
                },
                Vertex {
                    position: glm::vec3(0.0, 1.0, -phi),
                    material: material,
                },
                Vertex {
                    position: glm::vec3(phi, 0.0, -1.0),
                    material: material,
                },
                Vertex {
                    position: glm::vec3(phi, 0.0, 1.0),
                    material: material,
                },
                Vertex {
                    position: glm::vec3(-phi, 0.0, -1.0),
                    material: material,
                },
                Vertex {
                    position: glm::vec3(-phi, 0.0, 1.0),
                    material: material,
                },
            ],
            triangles: vec![
                Triangle::new(0, 11, 5),
                Triangle::new(0, 5, 1),
                Triangle::new(0, 1, 7),
                Triangle::new(0, 7, 10),
                Triangle::new(0, 10, 11),
                Triangle::new(1, 5, 9),
                Triangle::new(5, 11, 4),
                Triangle::new(11, 10, 2),
                Triangle::new(10, 7, 6),
                Triangle::new(7, 1, 8),
                Triangle::new(3, 9, 4),
                Triangle::new(3, 4, 2),
                Triangle::new(3, 2, 6),
                Triangle::new(3, 6, 8),
                Triangle::new(3, 8, 9),
                Triangle::new(4, 9, 5),
                Triangle::new(2, 4, 11),
                Triangle::new(6, 2, 10),
                Triangle::new(8, 6, 7),
                Triangle::new(9, 8, 1),
            ],
            mesh: Mesh {
                vertices: Vec::new(),
                indices: Vec::new(),
                normals: Vec::new(),
                material: MeshMaterial {
                    ambient: Vec::new(),
                    diffuse: Vec::new(),
                    specular: Vec::new(),
                    shininess: Vec::new(),
                },

                index_count: 0,
            },
            mesh_material: material,
            displaced_vertices: Vec::new(),
        };

        regular_isocahedron.subdivide(detail);
        regular_isocahedron.triangles.drain(0..20);
        regular_isocahedron.displaced_vertices = regular_isocahedron.vertices.clone();

        regular_isocahedron.generate_mesh();

        regular_isocahedron
    }

    fn add_vertex(&mut self, vertex: Vertex) -> usize {
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    fn generate_mesh(&mut self) {
        self.mesh = Mesh {
            vertices: self.flatten_vertices(),
            indices: self.flatten_cells(),
            normals: self.get_vertex_normals(),
            material: self.get_mesh_material(),

            index_count: (self.triangles.len() * 3) as i32,
        };
    }

    fn get_mesh_material(&self) -> MeshMaterial {
        MeshMaterial {
            ambient: self.flatten_ambient(),
            diffuse: self.flatten_diffuse(),
            specular: self.flatten_specular(),
            shininess: self.flatten_shininess(),
        }
    }

    fn flatten_vertices(&self) -> Vec<f32> {
        let mut vec = Vec::new();
        for vertex in &self.displaced_vertices {
            vec.extend(&vertex.position);
        }
        vec
    }

    fn flatten_ambient(&self) -> Vec<f32> {
        let mut vec = Vec::new();
        for vertex in &self.vertices {
            vec.extend(&vertex.material.ambient);
        }
        vec
    }

    fn flatten_diffuse(&self) -> Vec<f32> {
        let mut vec = Vec::new();
        for vertex in &self.vertices {
            vec.extend(&vertex.material.diffuse);
        }
        vec
    }

    fn flatten_specular(&self) -> Vec<f32> {
        let mut vec = Vec::new();
        for vertex in &self.vertices {
            vec.extend(&vertex.material.specular);
        }
        vec
    }

    fn flatten_shininess(&self) -> Vec<f32> {
        let mut vec = Vec::new();
        for vertex in &self.vertices {
            vec.push(vertex.material.shininess);
        }
        vec
    }

    fn flatten_cells(&self) -> Vec<u32> {
        let mut vec = Vec::new();
        for cell in &self.triangles {
            vec.extend_from_slice(&[cell.a as u32, cell.b as u32, cell.c as u32]);
        }
        vec
    }

    fn get_vertex_normals(&mut self) -> Vec<f32> {
        let mut vec: Vec<f32> = Vec::new();
        let mut vertex_normals: Vec<glm::Vec3> =
            vec![glm::vec3(0.0, 0.0, 0.0); self.vertices.len()];

        for cell in &self.triangles {
            let a = self.vertices[cell.a].position;
            let b = self.vertices[cell.b].position;
            let c = self.vertices[cell.c].position;

            let ab = b - a;
            let ac = c - a;
            let mut triangle_normal = glm::cross(&ab, &ac);

            triangle_normal = glm::normalize(&triangle_normal);

            vertex_normals[cell.a] += triangle_normal;
            vertex_normals[cell.b] += triangle_normal;
            vertex_normals[cell.c] += triangle_normal;
        }

        for normal in &vertex_normals {
            vec.extend(&glm::normalize(normal));
        }

        vec
    }

    fn subdivide(&mut self, detail: u32) {
        let triangle_copy = self.triangles.clone();
        for triangle in triangle_copy {
            let a = self.vertices[triangle.a].position;
            let b = self.vertices[triangle.b].position;
            let c = self.vertices[triangle.c].position;
            self.subdivide_triangle(a, b, c, detail);
        }
    }

    fn subdivide_triangle(&mut self, a: glm::Vec3, b: glm::Vec3, c: glm::Vec3, detail: u32) {
        let cols = 2usize.pow(detail);
        let mut new_vertices: Vec<Vec<glm::Vec3>> = vec![];

        for i in 0..=cols {
            new_vertices.push(vec![]);
            let aj = a.clone().lerp(c, i as f32 / cols as f32);
            let bj = b.clone().lerp(c, i as f32 / cols as f32);
            let rows = cols - i;

            for j in 0..=rows {
                if j == 0 && i == cols {
                    new_vertices[i].push(aj.normalize());
                } else {
                    new_vertices[i].push(aj.clone().lerp(bj, j as f32 / rows as f32).normalize());
                }
            }
        }

        for i in 0..cols {
            for j in 0..2 * (cols - i) - 1 {
                let k = j / 2;

                let mut triangle = Triangle { a: 0, b: 0, c: 0 };
                if j % 2 == 0 {
                    triangle.a = self.add_vertex(Vertex {
                        position: new_vertices[i][k + 1],
                        material: self.mesh_material,
                    });

                    triangle.b = self.add_vertex(Vertex {
                        position: new_vertices[i + 1][k],
                        material: self.mesh_material,
                    });

                    triangle.c = self.add_vertex(Vertex {
                        position: new_vertices[i][k],
                        material: self.mesh_material,
                    });
                } else {
                    triangle.a = self.add_vertex(Vertex {
                        position: new_vertices[i][k + 1],
                        material: self.mesh_material,
                    });

                    triangle.b = self.add_vertex(Vertex {
                        position: new_vertices[i + 1][k + 1],
                        material: self.mesh_material,
                    });

                    triangle.c = self.add_vertex(Vertex {
                        position: new_vertices[i + 1][k],
                        material: self.mesh_material,
                    });
                }

                self.triangles.push(triangle);
            }
        }
    }

    pub fn generate_with_new_detail(&mut self, detail: u32) {
        let new_sphere = Sphere::new(detail, self.mesh_material);

        self.vertices = new_sphere.vertices;
        self.triangles = new_sphere.triangles;
        self.mesh_material = new_sphere.mesh_material;
        self.displaced_vertices = new_sphere.displaced_vertices;
        self.mesh = new_sphere.mesh;
    }
}
