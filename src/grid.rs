pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<f32>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![0.0; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, v: f32) {
        self.cells[y * self.width + x] = v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_initializes_to_zero() {
        let grid = Grid::new(4, 3);
        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 3);
        assert!(grid.cells.iter().all(|&v| v == 0.0));
    }

    #[test]
    fn new_allocates_correct_cell_count() {
        let grid = Grid::new(5, 7);
        assert_eq!(grid.cells.len(), 35);
    }

    #[test]
    fn set_and_get_roundtrip() {
        let mut grid = Grid::new(4, 4);
        grid.set(2, 3, 1.5);
        assert_eq!(grid.get(2, 3), 1.5);
    }

    #[test]
    fn set_uses_row_major_indexing() {
        let mut grid = Grid::new(4, 4);
        grid.set(1, 2, 9.9);
        // y * width + x = 2 * 4 + 1 = 9
        assert_eq!(grid.cells[9], 9.9);
    }

    #[test]
    fn set_does_not_clobber_neighbors() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, 1.0);
        grid.set(1, 0, 2.0);
        grid.set(0, 1, 3.0);
        assert_eq!(grid.get(0, 0), 1.0);
        assert_eq!(grid.get(1, 0), 2.0);
        assert_eq!(grid.get(0, 1), 3.0);
    }
}
