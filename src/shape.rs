use lerp::Lerp;

#[derive(Copy, Clone)]
pub struct Material {
    pub ambient: glm::Vec3,
    pub diffuse: glm::Vec3,
    pub specular: glm::Vec3,
    pub shininess: f32,
}

pub struct Light {
    pub position: glm::Vec3,
    pub ambient: glm::Vec3,
    pub diffuse: glm::Vec3,
    pub specular: glm::Vec3,
}

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

pub struct Shape {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,

    pub ambient: Vec<f32>,
    pub diffuse: Vec<f32>,
    pub specular: Vec<f32>,
    pub shininess: Vec<f32>,

    pub index_count: i32,
}

impl Triangle {
    fn new(a: usize, b: usize, c: usize) -> Triangle {
        Triangle { a, b, c }
    }
}

pub struct Sphere {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
    pub shape: Shape,
    pub shape_material: Material,
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
            shape: Shape {
                vertices: Vec::new(),
                indices: Vec::new(),
                normals: Vec::new(),
                ambient: Vec::new(),
                diffuse: Vec::new(),
                specular: Vec::new(),
                shininess: Vec::new(),

                index_count: 0,
            },
            shape_material: material,
        };

        regular_isocahedron.subdivide(1.0, detail);
        regular_isocahedron.triangles.drain(0..20);
        regular_isocahedron.generate_render_shape();

        regular_isocahedron
    }

    fn add_vertex(&mut self, vertex: Vertex) -> usize {
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    fn generate_render_shape(&mut self) {
        self.shape = Shape {
            vertices: self.flatten_vertices(),
            indices: self.flatten_cells(),
            normals: self.get_vertex_normals(),
            ambient: self.flatten_ambient(),
            diffuse: self.flatten_diffuse(),
            specular: self.flatten_specular(),
            shininess: self.flatten_shininess(),

            index_count: (self.triangles.len() * 3) as i32,
        };
    }

    fn flatten_vertices(&self) -> Vec<f32> {
        let mut vec = Vec::new();
        for vertex in &self.vertices {
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

    fn subdivide(&mut self, radius: f32, detail: u32) {
        let triangle_copy = self.triangles.clone();
        for triangle in triangle_copy {
            let a = self.vertices[triangle.a].position;
            let b = self.vertices[triangle.b].position;
            let c = self.vertices[triangle.c].position;
            self.subdivide_triangle(a, b, c, radius, detail);
        }
    }

    fn subdivide_triangle(
        &mut self,
        a: glm::Vec3,
        b: glm::Vec3,
        c: glm::Vec3,
        radius: f32,
        detail: u32,
    ) {
        let cols = 2usize.pow(detail);
        let mut new_vertices: Vec<Vec<glm::Vec3>> = vec![];

        for i in 0..=cols {
            new_vertices.push(vec![]);
            let aj = a.clone().lerp(c, i as f32 / cols as f32);
            let bj = b.clone().lerp(c, i as f32 / cols as f32);
            let rows = cols - i;

            for j in 0..=rows {
                if j == 0 && i == cols {
                    new_vertices[i].push(aj.normalize() * radius);
                } else {
                    new_vertices[i]
                        .push(aj.clone().lerp(bj, j as f32 / rows as f32).normalize() * radius);
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
                        material: self.shape_material,
                    });

                    triangle.b = self.add_vertex(Vertex {
                        position: new_vertices[i + 1][k],
                        material: self.shape_material,
                    });

                    triangle.c = self.add_vertex(Vertex {
                        position: new_vertices[i][k],
                        material: self.shape_material,
                    });
                } else {
                    triangle.a = self.add_vertex(Vertex {
                        position: new_vertices[i][k + 1],
                        material: self.shape_material,
                    });

                    triangle.b = self.add_vertex(Vertex {
                        position: new_vertices[i + 1][k + 1],
                        material: self.shape_material,
                    });

                    triangle.c = self.add_vertex(Vertex {
                        position: new_vertices[i + 1][k],
                        material: self.shape_material,
                    });
                }

                self.triangles.push(triangle);
            }
        }
    }
}
