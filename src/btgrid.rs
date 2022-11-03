use raylib::prelude::Vector2;
use std::collections::BTreeMap;

pub struct Grid {
    cell_size: u16,
    cells: BTreeMap<(u16, u16), Vec<usize>>,
}

impl Grid {
    pub fn new(cell_size: u16) -> Grid {
        Grid {
            cell_size,
            cells: BTreeMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn add(&mut self, ent: usize, pos: Vector2, size: Vector2) {
        let low_x: usize = (pos.x as usize) / (self.cell_size as usize);
        let high_x: usize = ((pos.x + size.x) as usize) / (self.cell_size as usize);
        let low_y: usize = (pos.y as usize) / (self.cell_size as usize);
        let high_y: usize = ((pos.y + size.y) as usize) / (self.cell_size as usize);
        for y in low_y..=high_y {
            for x in low_x..=high_x {
                let key = (x as u16, y as u16);
                self.cells.entry(key).or_insert(vec![]).push(ent);
            }
        }
    }

    pub fn query(&mut self, pos: Vector2, size: Vector2) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        let low_x: usize = (pos.x as usize) / (self.cell_size as usize);
        let high_x: usize = ((pos.x + size.x) as usize) / (self.cell_size as usize);
        let low_y: usize = (pos.y as usize) / (self.cell_size as usize);
        let high_y: usize = ((pos.y + size.y) as usize) / (self.cell_size as usize);
        for y in low_y..=high_y {
            for x in low_x..=high_x {
                let key = (x as u16, y as u16);
                if let Some(cell) = self.cells.get(&key) {
                    result.extend_from_slice(cell);
                }
            }
        }
        result
    }

    pub fn get_cells_for_render(&self) -> Vec<Vector2> {
        let mut result: Vec<Vector2> = vec![];
        for key in self.cells.keys() {
            result.push(Vector2::new(
                key.0 as f32 * self.cell_size as f32,
                key.1 as f32 * self.cell_size as f32,
            ));
        }
        result
    }
}
