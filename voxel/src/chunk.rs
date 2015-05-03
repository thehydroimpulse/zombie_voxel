use block::{Material, Block};
use cgmath::Vector3;

const CHUNK_SIZE: usize = 500;

pub struct Chunk {
    // A sequentially allocated set of blocks. This vector holds
    // blocks where the index encodes the 3-dimensional position.
    //
    // As a result, the vector's size is n*3 where n = number of blocks in a single chunk.
    //
    // **Index Calculation:**
    //
    //  blocks[x + y + z] = block;
    blocks: Vec<Block>,
    // The base position for the entire chunk. Each block position is simply relative to it's
    // children based on it's index encoding and thus a relative position of the chunk's base
    // position.
    pos: Vector3<f32>
}

impl Chunk {
    pub fn new(pos: Vector3<f32>) -> Chunk {
        Chunk {
            blocks: Vec::with_capacity(CHUNK_SIZE * 3),
            pos: pos
        }
    }

    /// Return a block's position given it's index.
    // pub fn block_pos(&self) -> Vector3<f32> {

    // }

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
