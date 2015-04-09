use block::Block;

pub struct Chunk<B> {
    blocks: Vec<B>,
    // The position (x, y, z) of the chunk. Chunk
    // positions are completely independent of individual blocks.
    // Position 0, 0, 0 is the center block that becomes the starting
    // position.
    pos: (f32, f32, f32)
}

impl<B: Block> Chunk<B> {
    pub fn new(pos: (f32, f32, f32)) -> Chunk<B> {
        Chunk {
            blocks: Vec::new(),
            pos: pos
        }
    }

    pub fn add(&mut self, block: B) {
        self.blocks.push(block);
    }

    pub fn blocks(&self) -> &[B] {
        &self.blocks[..]
    }
}
