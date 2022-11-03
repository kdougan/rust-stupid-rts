use raylib::prelude::*;

mod grid;
mod entity;


fn main() {
    let (mut rl, thread) = raylib::init().size(500, 500).title("playgorund").build();

    rl.set_target_fps(60);

    let mut ents: Vec<entity::Entity> = Vec::new();
    let mut grid = grid::Grid::new(10);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // if key is down
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            for _ in 0..1000 {
                ents.push(entity::Entity::new(Vector2::new(250.0, 250.0), Vector2::new(10.0, 10.0)));
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
                        rand::random::<f32>() * 2.0 - 1.0
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
