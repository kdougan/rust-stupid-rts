use raylib::prelude::*;

#[derive(Debug)]
pub struct Entity {
    pub pos: Vector2,
    pub size: Vector2,
    pub acc: Vector2,
    pub vel: Vector2,
}

impl Entity {
    pub fn new(pos: Vector2, size: Vector2) -> Entity {
        Entity {
            pos,
            size,
            acc: Vector2::zero(),
            vel: Vector2::zero(),
        }
    }

    pub fn collides_with(&self, other: &Entity) -> bool {
        // AABB collides_with
        self.pos.x <= other.pos.x + other.size.x
            && self.pos.x + self.size.x >= other.pos.x
            && self.pos.y <= other.pos.y + other.size.y
            && self.pos.y + self.size.y >= other.pos.y
    }
}
