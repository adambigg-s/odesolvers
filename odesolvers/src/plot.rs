use crate::plot_utils::{Buffer, LineTracer};
use crate::scalar::Floating;
use crate::vec3::Vec3;

const BRAILLE_START: u16 = 0x2800_u16;
const BRAILLE_WIDTH: usize = 2_usize;
const BRAILLE_HEIGHT: usize = 4_usize;

const FOREGROUND_DEFAULT: Vec3<u8> = Vec3::build(0, 0, 255);
const BACKGROUND_DEFAULT: Vec3<u8> = Vec3::build(220, 220, 220);

const BOUNDS_DEFAULT: f32 = 20_f32;

pub struct Plot {
    plot: Buffer<Vec3<u8>>,
    xrange: Interval<f32>,
    yrange: Interval<f32>,
}

impl Plot {
    pub fn build(height: usize, width: usize) -> Self {
        Plot {
            plot: Buffer::build(height, width, BACKGROUND_DEFAULT),
            xrange: Interval::build(-BOUNDS_DEFAULT, BOUNDS_DEFAULT),
            yrange: Interval::build(-BOUNDS_DEFAULT, BOUNDS_DEFAULT),
        }
    }

    pub fn clear(&mut self) {
        self.plot.buff.fill(BACKGROUND_DEFAULT);
    }

    pub fn plot_point(&mut self, x: f32, y: f32) -> bool {
        if !self.within_plot(x, y) {
            return false;
        }
        let (plotx, ploty) = self.to_plotspace(x, y);
        self.plot.set(plotx, ploty, FOREGROUND_DEFAULT);
        true
    }

    pub fn plot_line<T>(&mut self, x0: T, y0: T, x1: T, y1: T)
    where
        T: Floating,
    {
        let (x0, y0, x1, y1) = (x0.to_f32(), y0.to_f32(), x1.to_f32(), y1.to_f32());
        if !self.within_plot(x0, y0) || !self.within_plot(x1, y1) {
            return;
        }
        let (x0, y0) = self.to_plotspace(x0, y0);
        let (x1, y1) = self.to_plotspace(x1, y1);
        let tracer = LineTracer::build(x0 as isize, y0 as isize, x1 as isize, y1 as isize);
        for (x, y) in tracer {
            self.plot.set(x as usize, y as usize, FOREGROUND_DEFAULT);
        }
    }

    pub fn display(&self) {
        let mut string = String::new();
        (0..self.plot.height).for_each(|y| {
            (0..self.plot.width).for_each(|x| {
                let chr = self.plot.get(x, y).unwrap();
                if chr != BACKGROUND_DEFAULT {
                    string.push('*');
                }
                else {
                    string.push(' ');
                }
            });
            string.push('\n');
        });

        println!("\x1b[0H{}", string);
    }

    fn within_plot(&self, x: f32, y: f32) -> bool {
        self.xrange.contains(x) && self.yrange.contains(y)
    }

    fn to_plotspace(&self, x: f32, y: f32) -> (usize, usize) {
        let xnorm = self.xrange.normalize(x);
        let ynorm = 1. - self.yrange.normalize(y);

        let xscreen = (xnorm * (self.plot.width as f32)).floor() as usize;
        let yscreen = (ynorm * (self.plot.height as f32)).floor() as usize;

        (xscreen, yscreen)
    }
}

pub struct Interval<T> {
    min: T,
    max: T,
}

impl<T> Interval<T>
where
    T: Floating + Copy,
{
    fn build(min: T, max: T) -> Interval<T> {
        Interval { min, max }
    }

    fn contains(&self, value: T) -> bool {
        (self.min..self.max).contains(&value)
    }

    fn normalize(&self, value: T) -> T {
        (value - self.min) / (self.max - self.min)
    }
}
