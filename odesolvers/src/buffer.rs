use std::io::Write;

use crate::vec3::Vec3;

const BRAILLE_START: u16 = 0x2800_u16;
const BRAILLE_WIDTH: usize = 2_usize;
const BRAILLE_HEIGHT: usize = 4_usize;

const FOREGROUND_DEFAULT: Vec3<u8> = Vec3::build(0, 0, 255);
const BACKGROUND_DEFAULT: Vec3<u8> = Vec3::build(240, 240, 240);

#[derive(Clone, Copy, PartialEq)]
enum DotState {
    Off,
    On,
}

pub struct Buffer {
    height: usize,
    width: usize,
    dots: Vec<DotState>,
    foreground: Vec3<u8>,
    background: Vec3<u8>,
}

impl Buffer {
    pub fn build(width_terminal: usize, height_terminal: usize) -> Self {
        let width = width_terminal * BRAILLE_WIDTH;
        let height = height_terminal * BRAILLE_HEIGHT;
        Buffer {
            height,
            width,
            dots: vec![DotState::Off; width * height],
            foreground: FOREGROUND_DEFAULT,
            background: BACKGROUND_DEFAULT,
        }
    }

    pub fn set_fg_color(&mut self, color: Vec3<u8>) -> &mut Self {
        self.foreground = color;
        self
    }

    pub fn set_bg_color(&mut self, color: Vec3<u8>) -> &mut Self {
        self.background = color;
        self
    }

    pub fn width_f(&self) -> f32 {
        self.width as f32
    }

    pub fn height_f(&self) -> f32 {
        self.height as f32
    }

    pub fn width_chars(&self) -> usize {
        self.width / BRAILLE_WIDTH
    }

    pub fn height_chars(&self) -> usize {
        self.height / BRAILLE_HEIGHT
    }

    pub fn clear(&mut self) {
        self.dots.iter_mut().for_each(|dot| *dot = DotState::Off)
    }

    pub fn set(&mut self, x: usize, y: usize) -> bool {
        if !self.inbounds(x, y) {
            return false;
        }

        let index = self.index(x, y);
        self.dots[index] = DotState::On;

        true
    }

    pub fn plot_points_2d<ItrX, ItrY>(&mut self, x_data: ItrX, y_data: ItrY)
    where
        ItrX: IntoIterator<Item = f32>,
        ItrY: IntoIterator<Item = f32>,
    {
        let xs: Vec<f32> = x_data.into_iter().collect();
        let ys: Vec<f32> = y_data.into_iter().collect();

        assert!(
            xs.len() == ys.len(),
            "dimensions must exactly match: dim[x] = {}, dim[y] = {}",
            xs.len(),
            ys.len()
        );

        let domain = self.min_max(&xs);
        let range = self.min_max(&ys);

        xs.iter().zip(ys.iter()).for_each(|(x, y)| {
            let (sx, sy) = self.aff_to_ss(*x, *y, domain, range);
            self.set(sx, sy);
        });
    }

    pub fn plot_linstrip_2d<ItrX, ItrY>(&mut self, x_data: ItrX, y_data: ItrY)
    where
        ItrX: IntoIterator<Item = f32>,
        ItrY: IntoIterator<Item = f32>,
    {
        let xs: Vec<f32> = x_data.into_iter().collect();
        let ys: Vec<f32> = y_data.into_iter().collect();

        assert!(
            xs.len() == ys.len(),
            "dimensions must exactly match: dim[x] = {}, dim[y] = {}",
            xs.len(),
            ys.len()
        );

        let domain = self.min_max(&xs);
        let range = self.min_max(&ys);

        for (p1, p2) in xs.iter().zip(ys.iter()).zip(xs.iter().skip(1).zip(ys.iter().skip(1))) {
            let (x0, y0) = p1;
            let (x1, y1) = p2;
            let (sx0, sy0) = self.aff_to_ss(*x0, *y0, domain, range);
            let (sx1, sy1) = self.aff_to_ss(*x1, *y1, domain, range);

            self.bresen_plot_line(sx0 as isize, sy0 as isize, sx1 as isize, sy1 as isize);
        }
    }

    pub fn bresen_plot_line(&mut self, mut x0: isize, mut y0: isize, x1: isize, y1: isize) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();

        #[rustfmt::skip]
        let sx = if x0 < x1 { 1 } else { -1 };
        #[rustfmt::skip]
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            self.set(x0 as usize, y0 as usize);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;

            if e2 >= dy {
                if x0 == x1 {
                    break;
                }
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                if y0 == y1 {
                    break;
                }
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn render(&self) {
        let foreground =
            format!("\x1b[38;2;{};{};{}m", self.foreground.x, self.foreground.y, self.foreground.z);
        let background =
            format!("\x1b[48;2;{};{};{}m", self.background.x, self.background.y, self.background.z);
        let reset = "\x1b[0m";
        print!("{}{}", foreground, background);
        print!("\x1b[?25l\x1b[0H{}\x1b[?25h", self.render_to_string());
        print!("{}", reset);
        std::io::stdout().flush().unwrap();
    }

    fn aff_to_ss(&self, x: f32, y: f32, domain: (f32, f32), range: (f32, f32)) -> (usize, usize) {
        let (xmin, xmax) = domain;
        let (ymin, ymax) = range;

        let xnorm = (x - xmin) / (xmax - xmin);
        let ynorm = (y - ymin) / (ymax - ymin);

        let xscreen = (xnorm * (self.width_f() - 1.)).round() as usize;
        let yscreen = (ynorm * (self.height_f() - 1.)).round() as usize;

        (xscreen, yscreen)
    }

    fn min_max(&self, data: &[f32]) -> (f32, f32) {
        data.iter()
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), value| (min.min(*value), max.max(*value)))
    }

    fn render_to_string(&self) -> String {
        let mut result = String::new();

        (0..self.height_chars()).for_each(|cy| {
            (0..self.width_chars()).for_each(|cx| {
                let chr = self.get_braille_char(cx, cy);
                result.push(chr);
            });
            result.push('\n');
        });

        result
    }

    fn get_dot(&self, x: usize, y: usize) -> Option<DotState> {
        if !self.inbounds(x, y) {
            return None;
        }

        let index = self.index(x, y);
        Some(self.dots[index])
    }

    fn get_braille_char(&self, cx: usize, cy: usize) -> char {
        let mut pattern = 0u8;

        (0..BRAILLE_HEIGHT).for_each(|dy| {
            (0..BRAILLE_WIDTH).for_each(|dx| {
                let dot_bit = self.get_dot_bit_position(dx, dy);

                let x = cx * BRAILLE_WIDTH + dx;
                let y = cy * BRAILLE_HEIGHT + dy;

                if let Some(dotstate) = self.get_dot(x, y) {
                    if dotstate == DotState::On {
                        pattern |= 1_u8 << dot_bit;
                    }
                }
            });
        });

        let unicode_char = BRAILLE_START + pattern as u16;
        char::from_u32(unicode_char as u32).unwrap_or(' ')
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

    fn index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}
