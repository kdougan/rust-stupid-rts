use raylib::prelude::Vector2;

pub struct Grid {
    cell_size: u16,
    cells: Vec<Vec<Vec<usize>>>,
    pub ent_count: usize,
}

impl Grid {
    pub fn new(cell_size: u16) -> Grid {
        Grid {
            cell_size,
            cells: vec![],
            ent_count: 0,
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for col in row {
                col.clear();
            }
        }
        self.ent_count = 0;
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
                self.ent_count += 1;
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

    pub fn get_entities(&self) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        for row in &self.cells {
            for col in row {
                col.iter().for_each(|ent| {
                    result.push(*ent);
                });
            }
        }
        result
    }
}
