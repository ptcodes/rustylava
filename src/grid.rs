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
