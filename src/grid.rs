/*
    2d vector grid of immutable entity pointers/refs
*/

use crate::entity::Entity;
use raylib::prelude::*;
use std::cmp::{max, min};

pub struct Grid<'entity> {
    pub pos: Vector2,
    pub size: Vector2,
    pub width_num_cells: u32,
    pub height_num_cells: u32,
    pub cells: Vec<Vec<Vec<&'entity Entity>>>,
    pub cell_size: Vector2,
}

#[derive(Debug)]
pub struct GridCoordBounds {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl<'entity> Grid<'entity> {
    pub fn new(
        pos: Vector2,
        size: Vector2,
        mut width_num_cells: u32,
        mut height_num_cells: u32,
    ) -> Self {
        if width_num_cells == 0 || height_num_cells == 0 {
            //  you are a bad person
            width_num_cells = 1;
            height_num_cells = 1;
        }
        Self {
            pos,
            size,
            width_num_cells,
            height_num_cells,
            cells: vec![vec![vec![]; width_num_cells as usize]; height_num_cells as usize],
            cell_size: Vector2::new(
                size.x / width_num_cells as f32,
                size.y / height_num_cells as f32,
            ),
        }
    }

    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut().flatten() {
            cell.clear();
        }
    }

    pub fn pos_to_grid_coord(&self, pos: &Vector2) -> (usize, usize) {
        let g_pos = (*pos - self.pos) / self.cell_size;
        (g_pos.x as usize, g_pos.y as usize)
    }

    pub fn get_coord_boundaries(&self, entity: &Entity) -> GridCoordBounds {
        let tl = self.pos_to_grid_coord(&entity.pos);
        let br = self.pos_to_grid_coord(&entity.get_br());
        GridCoordBounds {
            top: max(tl.1, 0),
            bottom: min(br.1, self.height_num_cells as usize - 1),
            left: max(tl.0, 0),
            right: min(br.0, self.width_num_cells as usize - 1),
        }
    }

    pub fn add(&mut self, entity: &'entity Entity) {
        let b = self.get_coord_boundaries(entity);
        for y in b.top..=b.bottom {
            for x in b.left..=b.right {
                self.cells[y][x].push(entity);
            }
        }
    }

    pub fn query(&self, entity: &Entity) -> Vec<&Entity> {
        let mut result = Vec::new();
        let b = self.get_coord_boundaries(entity);

        for y in b.top..=b.bottom {
            for x in b.left..=b.right {
                for other in &self.cells[y][x] {
                    if entity == *other {
                        continue;
                    }
                    result.push(*other);
                }
            }
        }

        result.sort_unstable();
        result.dedup();
        result
    }

    // pub fn print(&self) {}
}
