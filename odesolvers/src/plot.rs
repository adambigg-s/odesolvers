use std::io::Write;

use crate::plot_utils::Brush;
use crate::plot_utils::Buffer;
use crate::plot_utils::Cell;
use crate::plot_utils::Color;
use crate::plot_utils::Interval;
use crate::plot_utils::LineTracer;
use crate::scalar::Floating;
use crate::vec3::Vec3;

#[rustfmt::skip]
const BRAILLE_MAP: [[usize; 2]; 4] = [
    [0, 3],
    [1, 4],
    [2, 5],
    [6, 7],
];
const BRAILLE_START: u16 = 0x2800;
const BRAILLE_WIDTH: usize = 2;
const BRAILLE_HEIGHT: usize = 4;
const BRAILLE_COUNT: usize = BRAILLE_HEIGHT * BRAILLE_WIDTH;

const FOREGROUND_DEFAULT: Vec3<u8> = Vec3::build(0, 0, 255);
const BACKGROUND_DEFAULT: Vec3<u8> = Vec3::build(220, 220, 220);

const BOUNDS_DEFAULT: f32 = 20.;

pub struct Plot {
    pub plot: Buffer<Cell>,
    pub xrange: Interval<f32>,
    pub yrange: Interval<f32>,

    pub brush: Brush,

    pub output_string: String,
}

impl Plot {
    pub fn build(height: usize, width: usize) -> Self {
        Plot {
            plot: Buffer::build(
                height * BRAILLE_HEIGHT,
                width * BRAILLE_WIDTH,
                Cell { front: FOREGROUND_DEFAULT, back: BACKGROUND_DEFAULT, active: false },
            ),
            xrange: Interval::build(-BOUNDS_DEFAULT, BOUNDS_DEFAULT),
            yrange: Interval::build(-BOUNDS_DEFAULT, BOUNDS_DEFAULT),

            brush: Brush::build(FOREGROUND_DEFAULT, BACKGROUND_DEFAULT),

            output_string: String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.plot.buff.fill(Cell { front: self.brush.front, back: self.brush.back, active: false });
        self.output_string.clear();
    }

    pub fn alter_brush(&mut self) -> &mut Brush {
        &mut self.brush
    }

    pub fn brush_default(&mut self) {
        self.brush = Brush::build(FOREGROUND_DEFAULT, BACKGROUND_DEFAULT);
    }

    pub fn plot_point<T>(&mut self, x: T, y: T) -> bool
    where
        T: Floating,
    {
        let (x, y) = (x.to_f32(), y.to_f32());
        if !self.within_plot(x, y) {
            return false;
        }

        let (plotx, ploty) = self.to_plotspace(x, y);
        self.plot.set(plotx, ploty, Cell::build(self.brush.front, self.brush.back));

        true
    }

    pub fn plot_line<T>(&mut self, x0: T, y0: T, x1: T, y1: T) -> bool
    where
        T: Floating,
    {
        let (x0, y0, x1, y1) = (x0.to_f32(), y0.to_f32(), x1.to_f32(), y1.to_f32());
        if !self.within_plot(x0, y0) || !self.within_plot(x1, y1) {
            return false;
        }

        let (x0, y0) = self.to_plotspace(x0, y0);
        let (x1, y1) = self.to_plotspace(x1, y1);
        let tracer = LineTracer::build(x0 as isize, y0 as isize, x1 as isize, y1 as isize);
        tracer.for_each(|(x, y)| {
            self.plot.set(x as usize, y as usize, Cell::build(self.brush.front, self.brush.back));
        });

        true
    }

    pub fn display(&mut self) {
        let mut curr_front = self.brush.front;
        let mut curr_back = self.brush.back;
        self.output_string.push_str("\x1b[H");
        self.output_string.push_str(&self.brush.front.to_ansi_front());
        self.output_string.push_str(&self.brush.back.to_ansi_back());
        (0..self.plot.height).step_by(BRAILLE_HEIGHT).for_each(|y| {
            (0..self.plot.width).step_by(BRAILLE_WIDTH).for_each(|x| {
                let (front, back) = self.braille_average_color(x, y);
                if front != curr_front {
                    self.output_string.push_str(&front.to_ansi_front());
                    curr_front = front;
                }
                if back != curr_back {
                    self.output_string.push_str(&back.to_ansi_back());
                    curr_back = back;
                }
                self.output_string.push(self.to_braille(x, y));
            });
            self.output_string.push('\n');
        });
        self.output_string.push_str("\x1b[0m");

        println!("{}", self.output_string);
        std::io::stdout().flush().unwrap();
    }

    fn braille_average_color(&self, x: usize, y: usize) -> (Color, Color) {
        let (mut front, mut back) = (Vec3::zeros(), Vec3::zeros());
        let mut counter = 0;
        (0..BRAILLE_HEIGHT).for_each(|dy| {
            (0..BRAILLE_WIDTH).for_each(|dx| {});
        });

        (front, back)
    }

    fn to_braille(&self, x: usize, y: usize) -> char {
        let mut braille_byte: u8 = 0;
        (0..BRAILLE_HEIGHT).for_each(|dy| {
            (0..BRAILLE_WIDTH).for_each(|dx| {
                if self.plot.get_unchecked(x + dx, y + dy).active {
                    braille_byte |= 1 << self.braille_bit(dx, dy);
                }
            });
        });

        unsafe { char::from_u32_unchecked((BRAILLE_START + braille_byte as u16) as u32) }
    }

    fn braille_bit(&self, x: usize, y: usize) -> usize {
        BRAILLE_MAP[y][x]
    }

    fn within_plot(&self, x: f32, y: f32) -> bool {
        self.xrange.contains(x) && self.yrange.contains(y)
    }

    fn to_plotspace(&self, x: f32, y: f32) -> (usize, usize) {
        let xnorm = self.xrange.normalized_coordinate(x);
        let ynorm = 1. - self.yrange.normalized_coordinate(y);

        let xscreen = (xnorm * (self.plot.width as f32)).round() as usize;
        let yscreen = (ynorm * (self.plot.height as f32)).round() as usize;

        (xscreen, yscreen)
    }
}
