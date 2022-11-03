use raylib::prelude::Vector2;

pub struct Grid {
    cell_size: u16,
    cells: Vec<Vec<Vec<usize>>>,
}

impl Grid {
    pub fn new(cell_size: u16) -> Grid {
        Grid {
            cell_size,
            cells: vec![],
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for col in row {
                col.clear();
            }
        }
    }

    pub fn add(&mut self, ent: usize, pos: Vector2, size: Vector2) {
        let low_x: usize = (pos.x as usize) / (self.cell_size as usize);
        let high_x: usize = ((pos.x + size.x) as usize) / (self.cell_size as usize);
        let low_y: usize = (pos.y as usize) / (self.cell_size as usize);
        let high_y: usize = ((pos.y + size.y) as usize) / (self.cell_size as usize);
        for y in low_y..=high_y {
            if self.cells.len() <= y {
                self.cells.resize(y + 100, vec![]);
            }
            for x in low_x..=high_x {
                if self.cells[y].len() <= x {
                    self.cells[y].resize(x + 100, vec![]);
                }
                self.cells[y][x].push(ent);
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
            if self.cells.len() < y {
                continue;
            }
            for x in low_x..=high_x {
                if self.cells[y].len() < x {
                    continue;
                }
                self.cells[y][x].iter_mut().for_each(|ent| {
                    result.push(*ent);
                });
            }
        }
        result
    }

    pub fn get_cells_for_render(&self) -> Vec<Vector2> {
        let mut result: Vec<Vector2> = vec![];
        for y in 0..self.cells.len() {
            for x in 0..self.cells[y].len() {
                if self.cells[y][x].len() > 0 {
                    result.push(Vector2::new(
                        x as f32 * self.cell_size as f32,
                        y as f32 * self.cell_size as f32,
                    ));
                }
            }
        }
        result
    }
}
