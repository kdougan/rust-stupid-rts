use raylib::prelude::*;
use std::collections::{ HashMap, HashSet };
use rand;

#[derive(Debug)]
struct Entity {
    pos: Vector2,
    size: Vector2,
    acc: Vector2,
    vel: Vector2,
}

impl Entity {
    fn new(pos: Vector2, size: Vector2) -> Entity {
        Entity {
            pos: pos,
            size: size,
            acc: Vector2::zero(),
            vel: Vector2::zero(),
        }
    }

    fn collides_with(&self, other: &Entity) -> bool {
        let a: bool = self.pos.x < other.pos.x + other.size.x;
        let b: bool = self.pos.x + self.size.x > other.pos.x;
        let c: bool = self.pos.y < other.pos.y + other.size.y;
        let d: bool = self.size.y + self.pos.y > other.pos.y;
        return a && b && c && d;
    }
}

struct Grid {
    cell_size: u32,
    cells: HashMap<(i32, i32), HashSet<u32>>,
    entities: HashMap<u32, HashSet<(i32, i32)>>,
}

impl Grid {
    fn new(cell_size: u32) -> Grid {
        Grid {
            cell_size,
            cells: HashMap::new(),
            entities: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.cells.clear();
        self.entities.clear();
    }

    fn add(&mut self, ent: u32, pos: (f32, f32), size: (f32, f32)) {
        let low_x: i32 = (pos.0 as i32) / (self.cell_size as i32);
        let high_x: i32 = ((pos.0 + size.0) as i32) / (self.cell_size as i32);
        let low_y: i32 = (pos.1 as i32) / (self.cell_size as i32);
        let high_y: i32 = ((pos.1 + size.1) as i32) / (self.cell_size as i32);
        for x in low_x..=high_x {
            for y in low_y..=high_y {
                self.cells.entry((x, y)).or_default().insert(ent);
                self.entities.entry(ent).or_default().insert((x, y));
            }
        }
    }

    fn query(&self, ent: u32) -> HashSet<u32> {
        let mut result: HashSet<u32> = HashSet::new();
        for cell in self.entities.get(&ent).unwrap() {
            result.extend(self.cells.get(&cell).unwrap());
        }
        return result;
    }

    fn print(&self) {
        println!(
            "<Grid cell_size={} cells={} ents={}>",
            self.cell_size,
            self.cells.len(),
            self.entities.len()
        );
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(500, 500).title("playgorund").build();

    rl.set_target_fps(60);

    let mut ents: Vec<Entity> = Vec::new();
    let mut grid = Grid::new(10);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // if key is down
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            for _ in 0..10 {
                ents.push(Entity::new(Vector2::new(250.0, 250.0), Vector2::new(10.0, 10.0)));
            }
        }

        grid.clear();

        for i in 0..ents.len() {
            let ent = &ents[i];
            grid.add(i as u32, (ent.pos.x, ent.pos.y), (ent.size.x, ent.size.y));
        }

        for i in 0..ents.len() {
            for j in grid.query(i as u32).iter() {
                if i == (*j as usize) {
                    continue;
                }
                let ent: &Entity = &ents[i];
                let oth: &Entity = &ents[*j as usize];
                if !ent.collides_with(&oth) {
                    continue;
                }
                let mut v = oth.pos - ent.pos;
                if v.length() == 0.0 {
                    v = Vector2::new(
                        rand::random::<f32>() * 2.0 - 1.0,
                        rand::random::<f32>() * 2.0 - 1.0
                    );
                }
                ents[i].acc = ent.acc + v.normalized() * -50.0;
            }
        }

        for i in 0..ents.len() {
            let ent: &mut Entity = &mut ents[i];
            ent.vel = ent.vel + ent.acc * dt;
            ent.pos = ent.pos + ent.vel * dt;
            ent.vel = ent.vel * 0.8;
            ent.acc = Vector2::zero();
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(30, 20, 30, 255));
        let ent_count_text = format!("fps: {:?} entities: {:?}", d.get_fps(), ents.len());
        d.draw_text(&ent_count_text, 12, 12, 20, Color::RAYWHITE);

        for ent in &ents {
            d.draw_rectangle(
                ent.pos.x as i32,
                ent.pos.y as i32,
                ent.size.x as i32,
                ent.size.y as i32,
                Color::RED
            );
        }
    }
}
