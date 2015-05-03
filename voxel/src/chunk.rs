use block::{Material, Block};
use cgmath::Vector3;

pub struct Chunk {
    // size = 500
    // Vec::with_capacity(500);
    // blocks[x + y + z] = block;
    blocks: Vec<Block>,
    // The position (x, y, z) of the chunk. Chunk
    // positions are completely independent of individual blocks.
    // Position 0, 0, 0 is the center block that becomes the starting
    // position.
    pos: Vector3<f32>
}

impl Chunk {
    pub fn new(pos: Vector3<f32>) -> Chunk {
        Chunk {
            blocks: Vec::with_capacity(500),
            pos: pos
        }
    }

    #[inline]
    pub fn replace_index(&mut self, index: usize, block: Block) {
        self.blocks[index] = block;
    }

    #[inline]
    pub fn replace_pos(&mut self, pos: Vector3<f32>, block: Block) {
        let index = (pos.x + pos.y + pos.z) as usize;
        self.replace_index(index, block)
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks[..]
    }
}
