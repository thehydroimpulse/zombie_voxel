use cgmath::Vector3;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Grass,
    Dirt,
    Air
}

pub struct Block {
    material: Material
}
