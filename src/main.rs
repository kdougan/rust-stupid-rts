use raylib::prelude::*;

mod entity;
mod grid;

const WINDOW_SIZE: Vector2 = Vector2::new(500.0, 500.0);
const ENTITY_SIZE: Vector2 = Vector2::new(2.0, 2.0);
const CELL_SIZE: u16 = 5;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_SIZE.x as i32, WINDOW_SIZE.y as i32)
        .title("playgorund")
        .build();

    rl.set_target_fps(60);

    let mut ents: Vec<entity::Entity> = Vec::new();
    let mut grid = grid::Grid::new(CELL_SIZE);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // if key is down
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            for _ in 0..100 {
                ents.push(entity::Entity::new(WINDOW_SIZE * 0.5, ENTITY_SIZE));
            }
        }

        grid.clear();

        for (i, ent) in ents.iter_mut().enumerate() {
            grid.add(i, ent.pos, ent.size);
        }

        for i in 0..ents.len() {
            for j in grid.query(ents[i].pos, ents[i].size) {
                if i == j {
                    continue;
                }
                let ent = &ents[i];
                let oth = &ents[j];
                if !ent.collides_with(oth) {
                    continue;
                }
                let mut v = oth.pos - ent.pos;
                if v.length() == 0.0 {
                    v = Vector2::new(
                        rand::random::<f32>() * 2.0 - 1.0,
                        rand::random::<f32>() * 2.0 - 1.0,
                    );
                }
                ents[i].acc = ent.acc + v.normalized() * -50.0;
            }
        }

        for ent in &mut ents {
            ent.vel += ent.acc * dt;
            ent.pos += ent.vel * dt;
            ent.vel *= 0.98;
            ent.acc = Vector2::zero();
        }

        // keep ents within bounds
        for ent in &mut ents {
            if ent.pos.x < 0.0 {
                ent.pos.x = 0.0;
                ent.vel.x *= -1.0;
            }
            if ent.pos.x + ent.size.x > 500.0 {
                ent.pos.x = 500.0 - ent.size.x;
                ent.vel.x *= -1.0;
            }
            if ent.pos.y < 0.0 {
                ent.pos.y = 0.0;
                ent.vel.y *= -1.0;
            }
            if ent.pos.y + ent.size.y > 500.0 {
                ent.pos.y = 500.0 - ent.size.y;
                ent.vel.y *= -1.0;
            }
        }

        let mut d = rl.begin_drawing(&thread);

        for ent in &ents {
            d.draw_rectangle(
                ent.pos.x as i32,
                ent.pos.y as i32,
                ent.size.x as i32,
                ent.size.y as i32,
                Color::RED,
            );
        }

        d.clear_background(Color::new(30, 20, 30, 255));
        let ent_count_text = format!(
            "fps: {:?} ents: {:?} gents: {:?}",
            d.get_fps(),
            ents.len(),
            grid.ent_count
        );
        d.draw_text(&ent_count_text, 13, 13, 20, Color::BLACK);
        d.draw_text(&ent_count_text, 12, 12, 20, Color::RAYWHITE);
    }
}
