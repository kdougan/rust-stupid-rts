use std::collections::{HashMap, HashSet};

pub struct BGrid {
    cell_size: u32,
    cells: HashMap<(i32, i32), HashSet<u32>>,
    entities: HashMap<u32, HashSet<(i32, i32)>>,
}

impl BGrid {
    pub fn new(cell_size: u32) -> BGrid {
        BGrid {
            cell_size,
            cells: HashMap::new(),
            entities: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.entities.clear();
    }

    pub fn add(&mut self, ent: u32, pos: (f32, f32), size: (f32, f32)) {
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

    // TODO: is hashset the right way to check uniqueness
    pub fn query(&self, ent: u32) -> HashSet<u32> {
        let mut result: HashSet<u32> = HashSet::new();
        for cell in self.entities.get(&ent).unwrap() {
            result.extend(self.cells.get(&cell).unwrap());
        }
        return result;
    }

    pub fn print(&self) {
        println!(
            "<BGrid cell_size={} cells={} ents={}>",
            self.cell_size,
            self.cells.len(),
            self.entities.len()
        );
    }
}
