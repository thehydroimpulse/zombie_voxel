use std::rc::{Rc, Weak};
use std::fmt::Debug;
use std::fmt::Display;

pub type OctreeVec<T> = Vec<Octree<T>>;

#[derive(Display, Debug)]
pub struct Point(f32, f32, f32);

#[derive(Debug)]
pub struct Octree<T> {
    root: Node<T>
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    // center: Point,
    elem: Option<T>,
    child: Vec<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(nodes: Vec<Box<Node<T>>>) -> Node<T> {
        Node {
            elem: None,
            child: nodes
        }
    }

    pub fn from_block(block: T) -> Node<T> {
        Node {
            elem: Some(block),
            child: Vec::new()
        }
    }
}

impl<T> Octree<T> where T: Debug, T: Clone {
    pub fn new(blocks: &[T]) -> Octree<T> {
        Octree {
            root: Octree::combine_leafs(blocks)
        }
    }

    /// &[T] chunks 8
    /// for each block, create a new node.
    /// &[Node<T>; 8] -> Node<T> -> Vec<Node<T>>
    pub fn combine_leafs(blocks: &[T]) -> Node<T> {
        let mut groups = Vec::new();

        for chunk in blocks.chunks(8) {
            let mut chunk_nodes = Vec::with_capacity(8);

            for block in chunk.iter().cloned() {
                chunk_nodes.push(Box::new(Node::from_block(block)));
            }

            groups.push(Node::new(chunk_nodes));
        }

        return Octree::combine_groups(groups);
    }

    pub fn combine_groups(mut groups: Vec<Node<T>>) -> Node<T> {
        let mut new_groups = Vec::new();

        for chunk in groups.chunks(8) {
            let mut chunk_nodes = Vec::with_capacity(8);

            for block in chunk.iter().cloned() {
                chunk_nodes.push(Box::new(block));
            }

            new_groups.push(Node::new(chunk_nodes));
        }

        if (new_groups.len() == 1) {
            return new_groups.pop().unwrap();
        }

        return Octree::combine_groups(new_groups);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Debug;

    #[derive(Debug, Copy, Clone)]
    struct Block(bool);

    #[test]
    fn divide() {
        let blocks = &[
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),

            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),

            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),

            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),

            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),

            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),

            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),

            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true),
            Block(true)
        ];

        let tree = Octree::new(blocks);
        assert_eq!(tree.root.child.len(), 8);
    }
}
