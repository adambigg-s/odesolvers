pub struct Buffer {
    height: usize,
    width: usize,
    dots: Vec<bool>,
}

impl Buffer {
    pub fn build(width: usize, height: usize) -> Self {
        Buffer { height: height * 4, width: width * 2, dots: vec![false; width * 2 * height * 4] }
    }

    pub fn width(&self) -> f32 {
        (self.width / 2) as f32
    }

    pub fn height(&self) -> f32 {
        (self.height / 4) as f32
    }

    pub fn clear(&mut self) {
        self.dots.iter_mut().for_each(|datum| *datum = false);
    }

    pub fn set(&mut self, x: usize, y: usize) {
        if !(x < self.width && y < self.height) {
            return;
        }
        let index = self.index(x, y);
        self.dots[index] = true;
    }

    pub fn render(&self) {
        for cy in 0..(self.height / 4) {
            for cx in 0..(self.width / 2) {
                let mut braille = 0u8;

                for dy in 0..4 {
                    for dx in 0..2 {
                        let dot_idx = self.dot_index(dx, dy);
                        let x = cx * 2 + dx;
                        let y = cy * 4 + dy;

                        if x < self.width && y < self.height {
                            let index = self.index(x, y);
                            if self.dots[index] {
                                braille |= 1 << dot_idx;
                            }
                        }
                    }
                }

                let unicode_char = 0x2800u16 + braille as u16;
                print!("{}", char::from_u32(unicode_char as u32).unwrap_or(' '));
            }
            println!();
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn dot_index(&self, x: usize, y: usize) -> u8 {
        match (x, y) {
            (0, 0) => 0,
            (0, 1) => 1,
            (0, 2) => 2,
            (1, 0) => 3,
            (1, 1) => 4,
            (1, 2) => 5,
            (0, 3) => 6,
            (1, 3) => 7,
            _ => 0,
        }
    }
}
