pub struct Buffer<T> {
    pub buff: Vec<T>,
    pub height: usize,
    pub width: usize,
}

impl<T> Buffer<T>
where
    T: Copy + Clone,
{
    pub fn build(height: usize, width: usize, fill: T) -> Buffer<T> {
        Buffer { height, width, buff: vec![fill; height * width] }
    }

    pub fn set(&mut self, x: usize, y: usize, data: T) -> bool {
        if !self.inbounds(x, y) {
            return false;
        }

        let index = self.index(x, y);
        self.buff[index] = data;
        true
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if !self.inbounds(x, y) {
            return None;
        }

        let index = self.index(x, y);
        Some(self.buff[index])
    }

    fn index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

pub struct LineTracer {
    x0: isize,
    y0: isize,
    x1: isize,
    y1: isize,
    dx: isize,
    dy: isize,
    sx: isize,
    sy: isize,
    err: isize,
    done: bool,
}

impl LineTracer {
    pub fn build(x0: isize, y0: isize, x1: isize, y1: isize) -> Self {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        #[rustfmt::skip]
        let sx = if x0 < x1 { 1 } else { -1 };
        #[rustfmt::skip]
        let sy = if y0 < y1 { 1 } else { -1 };
        let err = dx + dy;

        LineTracer { x0, y0, x1, y1, dx, dy, sx, sy, err, done: false }
    }
}

impl Iterator for LineTracer {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let point = (self.x0, self.y0);

        if self.x0 == self.x1 && self.y0 == self.y1 {
            self.done = true;
        }

        let e2 = 2 * self.err;
        if e2 >= self.dy {
            self.err += self.dy;
            self.x0 += self.sx;
        }
        if e2 <= self.dx {
            self.err += self.dx;
            self.y0 += self.sy;
        }

        Some(point)
    }
}
