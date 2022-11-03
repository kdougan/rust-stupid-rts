use raylib::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Entity {
    pub pos: Vector2,
    pub size: Vector2,
    pub acc: Vector2,
    pub vel: Vector2,
}

impl Entity {
    pub fn new(pos: Vector2, size: Vector2) -> Self {
        Self {
            pos,
            size,
            acc: Vector2::zero(),
            vel: Vector2::zero(),
        }
    }

    pub fn collides_with(&self, other: Self) -> bool {
        let a = self.pos;
        let b = self.get_br();
        let c = other.pos;
        let d = other.get_br();
        !(b.x < c.x || a.x > d.x || b.y < c.y || a.y > d.y)
    }

    pub fn get_br(&self) -> Vector2 {
        Vector2::new(self.pos.x + self.size.x, self.pos.y + self.size.y)
    }

    // pub fn get_center(&self) -> Vector2 {
    //     return Vector2::new(
    //         self.pos.x + self.size.x / 2.0,
    //         self.pos.y + self.size.y / 2.0,
    //     );
    // }
}
