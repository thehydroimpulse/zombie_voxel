use std::fmt::Debug;
use std::fmt::Display;
use chunk::Chunk;
use block::Block;

#[derive(Display, Debug)]
pub struct Point(f32, f32, f32);

#[derive(Debug)]
pub struct Octree {
    root: Node
}

#[derive(Debug, Clone)]
pub struct Node {
    /// The index of the block in a separate data structure like
    /// a list/vector.
    id: usize,
    children: Vec<Node>
}

/// Maximum octree depth that will be computed unless
/// we hit the leaf nodes before the given maximum.
pub struct Depth(u16);

/// Determine if a voxel intersects within some arbitrary boundary.
/// This is used when we're constructing our octree and if a boundary
/// intersects, we'll proceed to subdivide it unless we hit our specified
/// depth limit.
fn intersected() -> bool {
    false
}

impl Node {
    pub fn new(nodes: Vec<Node>) -> Node {
        Node {
            id: 0,
            children: nodes
        }
    }

    pub fn from_index(index: usize) -> Node {
        Node {
            id: index,
            children: Vec::new()
        }
    }
}

impl Octree {
    pub fn new<B: Block>(chunk: &Chunk<B>, resolution: Depth) -> Octree {
        for (i, block) in chunk.blocks().iter().enumerate() {
            
        }

        Octree {
            root: Node {
                id: 0,
                children: Vec::new()
            }
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Debug;
    use block::Block;
    use chunk::Chunk;

    #[derive(Debug, Copy, Clone)]
    struct Atom(bool, (f32, f32, f32));

    impl Block for Atom {
        fn is_visible(&self) -> bool {
            self.0
        }

        fn pos(&self) -> (f32, f32, f32) {
            self.1
        }
    }

    #[test]
    fn divide() {
        let mut chunk = Chunk::new((0.0, 0.0, 0.0));

        for n in 0..96 {
            chunk.add(Atom(true, (n as f32, (n - 20) as f32, 0.0)));
        }

        let tree = Octree::new(&chunk, Depth(4));
    }
}
