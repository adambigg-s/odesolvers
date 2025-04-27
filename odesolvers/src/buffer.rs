pub struct Buffer {
    height: usize,
    width: usize,
    color: Vec<u32>,
}

impl Buffer {
    pub fn build(width: usize, height: usize) -> Self {
        Buffer { height, width, color: vec![0; width * height] }
    }

    pub fn width(&self) -> f32 {
        self.width as f32
    }

    pub fn height(&self) -> f32 {
        self.height as f32
    }

    pub fn clear(&mut self) {
        self.color.iter_mut().for_each(|datum| *datum = 0);
    }

    pub fn set(&mut self, x: usize, y: usize, data: u32) {
        if !(x < self.width && y < self.height) {
            return;
        }
        let index = self.index(x, y);
        self.color[index] = data;
    }

    fn index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    pub fn render(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.index(x, y);
                let value = self.color[index];
                if value == 0 {
                    print!(" ");
                }
                else {
                    print!("*");
                }
            }
            println!();
        }
    }
}
