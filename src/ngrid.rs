use raylib::prelude::*;

pub struct Grid {
    cell_size: u32,
    cells: HashMap<(i32, i32), Vec<>>,
    entities: HashMap<u32, Vec<(i32, i32)>>,
}

impl Grid {
    pub fn new(cell_size: u32) -> Grid {
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

    // ent should be a pointer to the entity
    fn add(&mut self, ent: T, pos: Vector2, size: Vector2) {
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

    fn print(&self) {    }
}
