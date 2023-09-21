use std::collections::HashMap;

use super::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Cell {
    pub color: Color,
    pub x: isize,
    pub y: isize
}

#[derive(Debug, Clone)]
pub struct Board {
    cells: HashMap<(isize, isize), Color>,
    pub max_x: isize,
    pub min_x: isize,
    pub max_y: isize,
    pub min_y: isize
}
impl Board {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            max_x: 0,
            min_x: 0,
            max_y: 0,
            min_y: 0
        }
    }
    pub fn get(&self, pos: &(isize, isize)) -> Color {
        match self.cells.get(pos) {
            Some(c) => *c,
            None => Color::None
        }
    } 
    pub fn get_cell(&self, pos: &(isize, isize)) -> Cell {
        Cell {
            color: self.get(pos),
            x: pos.0,
            y: pos.1
        }
    }
    pub fn set(&mut self, pos: (isize, isize), color: Color) {
        if self.cells.is_empty() {
            self.max_x = pos.0;
            self.min_x = pos.0;
            self.max_y = pos.1;
            self.min_y = pos.1;
        } else {
            self.max_x = self.max_x.max(pos.0);
            self.min_x = self.min_x.min(pos.0);
            self.max_y = self.max_y.max(pos.1);
            self.min_y = self.min_y.min(pos.1);
        }
        self.cells.insert(pos, color);
    }
    pub fn remove(&mut self, pos: (isize, isize)) {
        self.cells.remove(&pos);
    }
    pub fn iter_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        self.cells.iter().map(|(pos, color)| Cell {
            color: *color,
            x: pos.0,
            y: pos.1
        })
    }
}
