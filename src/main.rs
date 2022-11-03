// use rand;
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

// #![warn(clippy::restriction)]
// #![warn(clippy::cargo)]
use raylib::prelude::*;
mod entity;
mod grid;

fn main() {
    let window_size = Vector2::new(800.0, 600.0);
    let window_center: Vector2 = window_size / 2.0;

    let (mut rl, thread) = raylib::init()
        .size(window_size.x as i32, window_size.y as i32)
        .title("playgorund")
        .build();

    rl.set_target_fps(60);

    let mut ents: Vec<entity::Entity> = Vec::new();
    const GRID_DENSITY: u32 = 16;
    let mut grid = grid::Grid::new(
        Vector2::new(0.0, 0.0),
        window_size,
        GRID_DENSITY,
        GRID_DENSITY,
    );

    const S: f32 = 20.0;
    const SIZE: Vector2 = Vector2::new(S, S);
    let pos = window_center + Vector2::new(-S, -S) / 2.0;
    ents.push(entity::Entity::new(pos, SIZE));

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // if key is down
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            for _ in 0..100 {
                ents.push(entity::Entity::new(window_size / 2.0, SIZE));
            }
        }

        // get mouse pos
        let mouse_pos = rl.get_mouse_position();
        ents[0].pos = mouse_pos - SIZE / 2.0;

        // add entities to grid
        grid.clear();
        for (i, ent) in ents.iter().enumerate() {
            grid.add(i, ent);
        }

        // do collisions
        for i in 0..ents.len() {
            let ent = ents[i];
            let mut acc = Vector2::zero();
            let query_result = grid.query(i, &ent);
            for j in &query_result {
                let j = *j;
                if i == (j) {
                    continue;
                }
                let oth = ents[j];
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
                acc += v.normalized() * -50.0;
            }
            {
                let mut ent = &mut ents[i];
                ent.acc = acc;
            }
        }

        for ent in &mut ents {
            ent.vel += ent.acc * dt;
            ent.pos += ent.vel * dt;
            ent.vel *= 0.8;
            ent.acc = Vector2::zero();
        }

        let mut d = rl.begin_drawing(&thread);

        //// draw grid
        // let cell_size =
        //     grid.size / Vector2::new(grid.width_num_cells as f32, grid.height_num_cells as f32);
        // for y in 0..grid.height_num_cells {
        //     for x in 0..grid.width_num_cells {
        //         let pos = grid.pos + Vector2::new(x as f32, y as f32) * cell_size;
        //         d.draw_rectangle_lines(
        //             pos.x as i32,
        //             pos.y as i32,
        //             cell_size.x as i32,
        //             cell_size.y as i32,
        //             Color::GRAY,
        //         );
        //     }
        // }
        // cell should be grey if it has any entities in it
        // if grid.cells[y as usize][x as usize].len() > 0 {
        //     // draw outline
        //     d.draw_rectangle(
        //         pos.x as i32,
        //         pos.y as i32,
        //         cell_size.x as i32,
        //         cell_size.y as i32,
        //         Color::DARKGRAY,
        //     );
        // } else {
        //     d.draw_rectangle_lines(
        //         pos.x as i32,
        //         pos.y as i32,
        //         cell_size.x as i32,
        //         cell_size.y as i32,
        //         Color::GRAY,
        //     );
        // };

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
        let ent_count_text = format!("fps: {:?} entities: {:?}", d.get_fps(), ents.len());
        d.draw_text(&ent_count_text, 12, 12, 20, Color::RAYWHITE);
    }
}
