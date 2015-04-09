/// A Block trait that is implemented for each block type.
pub trait Block {
    fn pos(&self) -> (f32, f32, f32);

    fn pos_x(&self) -> f32 {
        let (x, _, _) = self.pos();
        x
    }

    fn pos_y(&self) -> f32 {
        let (_, y, _) = self.pos();
        y
    }

    fn pos_z(&self) -> f32 {
        let (_, _, z) = self.pos();
        z
    }

    fn is_visible(&self) -> bool {
        false
    }
}


