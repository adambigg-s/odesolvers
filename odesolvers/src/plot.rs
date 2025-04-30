use crate::vec3::Vec3;

const BRAILLE_START: u16 = 0x2800_u16;
const BRAILLE_WIDTH: usize = 2_usize;
const BRAILLE_HEIGHT: usize = 4_usize;

const FOREGROUND_DEFAULT: Vec3<u8> = Vec3::build(0, 0, 255);
const BACKGROUND_DEFAULT: Vec3<u8> = Vec3::build(220, 220, 220);

pub struct Buffer<T> {
    height: usize,
    width: usize,
    buff: Vec<T>,
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

    pub fn index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

pub struct Plot {
    tbuffer: Buffer<Option<char>>,
    dotbuffer: Buffer<Option<Dot>>,
    renderbuffer: Buffer<char>,
    outputstring: String,

    brush: Brush,

    domain: (f32, f32),
    range: (f32, f32),
}

impl Plot {
    pub fn build(height: usize, width: usize) -> Self {
        Plot {
            tbuffer: Buffer::build(height, width, None),
            dotbuffer: Buffer::build(height * BRAILLE_HEIGHT, width * BRAILLE_WIDTH, None),
            renderbuffer: Buffer::build(height, width, ' '),
            outputstring: String::new(),

            brush: Brush::build(FOREGROUND_DEFAULT, BACKGROUND_DEFAULT),

            domain: (-1., 1.),
            range: (-1., 1.),
        }
    }

    pub fn set_space(&mut self, domain: (f32, f32), range: (f32, f32)) -> &mut Self {
        self.domain = domain;
        self.range = range;
        self
    }

    pub fn to_dotspace(&self, x: f32, y: f32) -> Option<(isize, isize)> {
        let w = (self.dotbuffer.width - 1) as f32;
        let h = (self.dotbuffer.height - 1) as f32;
        let (xmin, xmax) = self.domain;
        let (ymin, ymax) = self.range;

        let sx = ((x - xmin) / (xmax - xmin)) * w;
        let sy = ((y - ymin) / (ymax - ymin)) * h;

        Some((sx.round() as isize, (h - sy).round() as isize))
    }

    pub fn plot_world(&mut self, x: f32, y: f32) {
        if let Some((ix, iy)) = self.to_dotspace(x, y) {
            self.plot_dot(ix, iy);
        }
    }

    pub fn plot_dot(&mut self, x: isize, y: isize) {
        self.dotbuffer.set(x as usize, y as usize, Some(Dot::build(self.brush.fcolor, self.brush.bcolor)));
    }

    pub fn draw_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize) {
        for (x, y) in LineTracer::build(x0, y0, x1, y1) {
            self.plot_dot(x, y);
        }
    }

    fn braille_char(&self, cellx: usize, celly: usize) -> char {
        let mut mask: u8 = 0;
        for dy in 0..BRAILLE_HEIGHT {
            for dx in 0..BRAILLE_WIDTH {
                let px = cellx * BRAILLE_WIDTH + dx;
                let py = celly * BRAILLE_HEIGHT + dy;
                if let Some(Some(dot)) = self.dotbuffer.get(px, py) {
                    let bit_index = self.get_dot_bit_position(dx, dy);
                    mask |= 1 << bit_index;
                }
            }
        }
        let codepoint = BRAILLE_START + mask as u16;
        std::char::from_u32(codepoint as u32).unwrap_or(' ')
    }

    pub fn render(&mut self) -> String {
        for y in 0..self.tbuffer.height {
            for x in 0..self.tbuffer.width {
                let ch = self.braille_char(x, y);
                self.tbuffer.set(x, y, Some(ch));
            }
        }
        let mut out = String::new();
        for y in 0..self.tbuffer.height {
            for x in 0..self.tbuffer.width {
                out.push(self.tbuffer.get(x, y).unwrap().unwrap_or(' '));
            }
            out.push('\n');
        }
        out
    }

    fn get_dot_bit_position(&self, x: usize, y: usize) -> u8 {
        // these are just pulled from a table on wikipedia - gives the bit
        // position of the 8 avaliable dots to be filled in
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

#[derive(Clone, Copy)]
pub struct Dot {
    fcolor: Vec3<u8>,
    bcolor: Vec3<u8>,
}

impl Dot {
    pub fn build(fcolor: Vec3<u8>, bcolor: Vec3<u8>) -> Self {
        Dot { fcolor, bcolor }
    }
}

pub struct Brush {
    fcolor: Vec3<u8>,
    bcolor: Vec3<u8>,
}

impl Brush {
    pub fn build(fcolor: Vec3<u8>, bcolor: Vec3<u8>) -> Self {
        Brush { fcolor, bcolor }
    }
}

pub struct LineTracer {
    x: isize,
    y: isize,
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

        LineTracer { x: x0, y: y0, dx, dy, sx, sy, err, done: false }
    }
}

impl Iterator for LineTracer {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let point = (self.x, self.y);
        if self.x == self.x + self.dx && self.y == self.y - self.dy {
            self.done = true;
        }

        let e2 = 2 * self.err;
        if e2 >= self.dy {
            self.err += self.dy;
            self.x += self.sx;
        }
        if e2 <= self.dx {
            self.err += self.dx;
            self.y += self.sy;
        }

        Some(point)
    }
}
