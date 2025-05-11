use crate::plot_utils::Buffer;
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

    pub fn set_point(&mut self, x: f32, y: f32) -> bool {
        if !self.in_range(x, y) {
            return false;
        }
        let (px, py) = self.to_ss(x, y);
        self.plot.set(px, py, FOREGROUND_DEFAULT);
        true
    }

    pub fn in_range(&self, x: f32, y: f32) -> bool {
        self.xrange.contains(x) && self.yrange.contains(y)
    }

    pub fn display(&self) {
        println!("\x1b[0H{}", self.to_string());
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        (0..self.plot.height).for_each(|y| {
            (0..self.plot.width).for_each(|x| {
                let chr = self.plot.get(x, y).unwrap();
                if chr == FOREGROUND_DEFAULT {
                    result.push('#');
                }
                else {
                    result.push(' ');
                }
            });
            result.push('\n');
        });

        result
    }

    fn to_ss(&self, x: f32, y: f32) -> (usize, usize) {
        let xnorm = (x - self.xrange.min) / (self.xrange.max - self.xrange.min);
        let ynorm = 1. - (y - self.yrange.min) / (self.yrange.max - self.yrange.min);

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
    T: PartialOrd + Copy,
{
    fn build(min: T, max: T) -> Interval<T> {
        Interval { min, max }
    }

    fn contains(&self, value: T) -> bool {
        (self.min..self.max).contains(&value)
    }
}
