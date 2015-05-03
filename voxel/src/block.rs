use cgmath::Vector3;
use chunk::Chunk;
use shaders::Vertex;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Grass,
    Dirt,
    Air
}

#[derive(Clone, Copy)]
pub struct Block {
    material: Material
}

impl Block {
    pub fn new(mat: Material) -> Block {
        Block {
            material: mat
        }
    }

    /// For each block, convert them into an array of vertices.
    pub fn into_vertex(&self) -> [Vertex; 24] {
        [
            // top (0, 0, 1)
            Vertex { pos: [-1.0, -1.0, 1.0] },
            Vertex { pos: [1.0, -1.0, 1.0] },
            Vertex { pos: [1.0, 1.0, 1.0] },
            Vertex { pos: [-1.0, 1.0, 1.0] },
            // bottom (0, 0, -1)
            Vertex { pos: [-1.0, 1.0, -1.0] },
            Vertex { pos: [1.0, 1.0, -1.0] },
            Vertex { pos: [1.0, -1.0, -1.0] },
            Vertex { pos: [-1.0, -1.0, -1.0] },
            // right (1, 0, 0)
            Vertex { pos: [1.0, -1.0, -1.0] },
            Vertex { pos: [1.0, 1.0, -1.0] },
            Vertex { pos: [1.0, 1.0, 1.0] },
            Vertex { pos: [1.0, -1.0, 1.0] },
            // left (-1, 0, 0)
            Vertex { pos: [-1.0, -1.0, 1.0] },
            Vertex { pos: [-1.0, 1.0, 1.0] },
            Vertex { pos: [-1.0, 1.0, -1.0] },
            Vertex { pos: [-1.0, -1.0, -1.0] },
            // front (0, 1, 0)
            Vertex { pos: [1.0, 1.0, -1.0] },
            Vertex { pos: [-1.0, 1.0, -1.0] },
            Vertex { pos: [-1.0, 1.0, 1.0] },
            Vertex { pos: [1.0, 1.0, 1.0] },
            // back (0, -1, 0)
            Vertex { pos: [1.0, -1.0, 1.0] },
            Vertex { pos: [-1.0, -1.0, 1.0] },
            Vertex { pos: [-1.0, -1.0, -1.0] },
            Vertex { pos: [1.0, -1.0, -1.0] },
        ]
    }
}
