use std::io::Write;

use crate::plot_utils::Brush;
use crate::plot_utils::Buffer;
use crate::plot_utils::Cell;
use crate::plot_utils::Color;
use crate::plot_utils::Interval;
use crate::plot_utils::LineTracer;
use crate::plot_utils::PlotSettings;
use crate::scalar::Floating;
use crate::vector::Vec3;

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

    brush: Brush,

    output_string: String,

    settings: PlotSettings,
}

impl Plot {
    const EPSILON: f32 = 1e-3;

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

            output_string: String::from("\x1b[2J"),

            settings: PlotSettings {
                axis: true,
                subtick: false,
                subtick_spacing: 0.,

                title: false,
                xlabel: false,
                ylabel: false,
            },
        }
    }

    pub fn xbounds(&mut self, min: f32, max: f32) -> &mut Self {
        self.xrange.min = min;
        self.xrange.max = max;
        self
    }

    pub fn ybounds(&mut self, min: f32, max: f32) -> &mut Self {
        self.yrange.min = min;
        self.yrange.max = max;
        self
    }

    pub fn set_brush(&mut self) -> &mut Brush {
        &mut self.brush
    }

    pub fn brush_default(&mut self) {
        self.brush = Brush::build(FOREGROUND_DEFAULT, BACKGROUND_DEFAULT);
    }

    pub fn set_settings(&mut self) -> &mut PlotSettings {
        &mut self.settings
    }

    pub fn clear(&mut self) {
        self.plot.buff.fill(Cell { front: self.brush.front, back: self.brush.back, active: false });
        self.output_string.clear();
        self.apply_settings();
    }

    pub fn new_plot(&mut self) {
        self.clear();
        (0..self.plot.height).step_by(BRAILLE_HEIGHT).for_each(|_| {
            println!();
        });
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
        self.output_string.push_str("\x1b[H\x1b[?25l");
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
        self.output_string.push_str("\x1b[?25h\x1b[0m");

        println!("{}", self.output_string);
        std::io::stdout().flush().unwrap();
    }

    fn apply_settings(&mut self) {
        if self.settings.subtick {
            self.draw_subticks();
        }
        if self.settings.axis {
            self.draw_axis();
        }
    }

    fn draw_axis(&mut self) {
        self.set_brush().front_color(100, 100, 100);
        self.plot_line(self.xrange.min + Self::EPSILON, 0., self.xrange.max - Self::EPSILON, 0.);
        self.plot_line(0., self.yrange.min + Self::EPSILON, 0., self.yrange.max - Self::EPSILON);
    }

    fn draw_subticks(&mut self) {
        self.set_brush().front_color(200, 200, 200);
        let mut axis = 0.;
        while axis < self.xrange.max {
            axis += self.settings.subtick_spacing;
            self.plot_line(axis, self.yrange.min + Self::EPSILON, axis, self.yrange.max - Self::EPSILON);
        }
        axis = 0.;
        while axis > self.xrange.min {
            axis -= self.settings.subtick_spacing;
            self.plot_line(axis, self.yrange.min + Self::EPSILON, axis, self.yrange.max - Self::EPSILON);
        }
        axis = 0.;
        while axis < self.yrange.max {
            axis += self.settings.subtick_spacing;
            self.plot_line(self.xrange.min + Self::EPSILON, axis, self.xrange.max - Self::EPSILON, axis);
        }
        axis = 0.;
        while axis > self.yrange.min {
            axis -= self.settings.subtick_spacing;
            self.plot_line(self.xrange.min + Self::EPSILON, axis, self.xrange.max - Self::EPSILON, axis);
        }
    }

    fn braille_average_color(&self, x: usize, y: usize) -> (Color, Color) {
        let (mut front, mut back) = (Vec3::<u16>::zeros(), Vec3::<u16>::zeros());
        let mut counter = 0;
        (0..BRAILLE_HEIGHT).for_each(|dy| {
            (0..BRAILLE_WIDTH).for_each(|dx| {
                if self.plot.get_unchecked(x + dx, y + dy).active {
                    front += self.plot.get_unchecked(x + dx, y + dy).front.cast();
                    counter += 1;
                }
                back += self.plot.get_unchecked(x + dx, y + dy).back.cast();
            });
        });
        front /= counter.max(1);
        back /= BRAILLE_COUNT as u16;

        (
            Vec3::build(front.x as u8, front.y as u8, front.z as u8),
            Vec3::build(back.x as u8, back.y as u8, back.z as u8),
        )
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

pub fn color_gradient(time: f32) -> (u8, u8, u8) {
    let red = (0.3 + 0.7 * time.sin()) * 255.;
    let green = (0.3 + 0.7 * (2. + time).sin()) * 255.;
    let blue = (0.3 + 0.7 * (4. + time).sin()) * 255.;

    (red as u8, green as u8, blue as u8)
}

pub fn wait(time_ms: u64) -> bool {
    std::thread::sleep(std::time::Duration::from_millis(time_ms));
    true
}
