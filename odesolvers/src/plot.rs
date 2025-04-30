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

    pub fn set_fcolor(&mut self, color: Vec3<u8>) -> &mut Self {
        self.fcolor = color;
        self
    }

    pub fn set_bcolor(&mut self, color: Vec3<u8>) -> &mut Self {
        self.bcolor = color;
        self
    }

    pub fn get_fcolor(&self) -> Vec3<u8> {
        self.fcolor
    }

    pub fn get_bcolor(&self) -> Vec3<u8> {
        self.bcolor
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

pub struct Plot {
    dotbuffer: Buffer<Option<Dot>>,
    renderbuffer: Buffer<(char, Vec3<u8>, Vec3<u8>)>,
    outputstring: String,

    brush: Brush,

    domain: (f32, f32),
    range: (f32, f32),
}

impl Plot {
    pub fn build(height: usize, width: usize) -> Self {
        Plot {
            dotbuffer: Buffer::build(height * BRAILLE_HEIGHT, width * BRAILLE_WIDTH, None),
            renderbuffer: Buffer::build(height, width, (' ', FOREGROUND_DEFAULT, BACKGROUND_DEFAULT)),
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

    pub fn plot_point(&mut self, x: f32, y: f32) {
        let (sx, sy) = self.bufferspace(x, y, &self.dotbuffer);
        self.dotbuffer.set(sx, sy, Some(Dot::build(self.brush.get_fcolor(), self.brush.get_bcolor())));
    }

    pub fn render(&mut self) {
        self.outputstring.clear();
        self.outputstring.push_str("\x1b[2J");
        self.outputstring.push_str("\x1b[H");

        for y in 0..self.renderbuffer.height {
            for x in 0..self.renderbuffer.width {
                let ch = self.get_braille_ascii(x, y);
                self.renderbuffer.set(x, y, (ch, self.brush.fcolor, self.brush.bcolor));
            }
        }

        for y in 0..self.renderbuffer.height {
            for x in 0..self.renderbuffer.width {
                if let Some((ch, fg, bg)) = self.renderbuffer.get(x, y) {
                    self.outputstring.push(ch);
                }
            }
            self.outputstring.push('\n');
        }

        println!("{}", self.outputstring);
    }

    pub fn get_braille_ascii(&self, x: usize, y: usize) -> char {
        let mut braille_byte = 0u8;
        for dy in 0..BRAILLE_HEIGHT {
            for dx in 0..BRAILLE_WIDTH {
                let gx = x * BRAILLE_WIDTH + dx;
                let gy = y * BRAILLE_HEIGHT + dy;

                if let Some(Some(_dot)) = self.dotbuffer.get(gx, gy) {
                    let bit = match (dx, dy) {
                        (0, 0) => 0,
                        (0, 1) => 1,
                        (0, 2) => 2,
                        (1, 0) => 3,
                        (1, 1) => 4,
                        (1, 2) => 5,
                        (0, 3) => 6,
                        (1, 3) => 7,
                        _ => 0,
                    };
                    braille_byte |= 1 << bit;
                }
            }
        }

        std::char::from_u32(BRAILLE_START as u32 + braille_byte as u32).unwrap_or(' ')
    }

    pub fn bufferspace<T>(&self, x: f32, y: f32, buff: &Buffer<T>) -> (usize, usize) {
        let (xmin, xmax) = self.domain;
        let (ymin, ymax) = self.range;

        let xnorm = (x - xmin) / (xmax - xmin);
        let ynorm = 1. - (y - ymin) / (ymax - ymin);

        let sx = (xnorm * (buff.width as f32 - 1.)).round() as usize;
        let sy = (ynorm * (buff.height as f32 - 1.)).round() as usize;

        (sx, sy)
    }
}
