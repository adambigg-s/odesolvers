use crate::plot_utils::Brush;
use crate::plot_utils::Buffer;
use crate::plot_utils::Cell;
use crate::plot_utils::Interval;
use crate::plot_utils::LineTracer;
use crate::scalar::Floating;
use crate::vec3::Vec3;

const BRAILLE_START: u16 = 0x2800;
const BRAILLE_WIDTH: usize = 2;
const BRAILLE_HEIGHT: usize = 4;

const FOREGROUND_DEFAULT: Vec3<u8> = Vec3::build(0, 0, 255);
const BACKGROUND_DEFAULT: Vec3<u8> = Vec3::build(220, 220, 220);

const BOUNDS_DEFAULT: f32 = 20.;

pub struct Plot {
    pub plot: Buffer<Option<Cell>>,
    pub xrange: Interval<f32>,
    pub yrange: Interval<f32>,

    pub brush: Brush,

    pub output_string: String,
}

impl Plot {
    pub fn build(height: usize, width: usize) -> Self {
        Plot {
            plot: Buffer::build(height, width, None),
            xrange: Interval::build(-BOUNDS_DEFAULT, BOUNDS_DEFAULT),
            yrange: Interval::build(-BOUNDS_DEFAULT, BOUNDS_DEFAULT),

            brush: Brush::build(FOREGROUND_DEFAULT, BACKGROUND_DEFAULT),

            output_string: String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.plot.buff.fill(None);
        self.output_string.clear();
    }

    pub fn alter_brush(&mut self) -> &mut Brush {
        &mut self.brush
    }

    pub fn brush_default(&mut self) {
        self.alter_brush().front_vec(FOREGROUND_DEFAULT).back_vec(BACKGROUND_DEFAULT);
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
        self.plot.set(plotx, ploty, Some(Cell::build(self.brush.fg, self.brush.bg)));

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
        for (x, y) in tracer {
            self.plot.set(x as usize, y as usize, Some(Cell::build(self.brush.fg, self.brush.bg)));
        }

        true
    }

    pub fn display(&mut self) {
        let mut last_front = self.brush.fg;
        let mut last_back = self.brush.bg;
        self.output_string.push_str("\x1b[0H");
        self.output_string.push_str(&self.brush.fg.to_ansi_front());
        self.output_string.push_str(&self.brush.bg.to_ansi_back());
        (0..self.plot.height).for_each(|y| {
            (0..self.plot.width).for_each(|x| {
                if let Some(cell) = self.plot.get_unchecked(x, y) {
                    let (front, back) = (cell.front, cell.back);
                    if last_front != front {
                        self.output_string.push_str(&front.to_ansi_front());
                        last_front = front;
                    }
                    if last_back != back {
                        self.output_string.push_str(&back.to_ansi_back());
                        last_back = back;
                    }
                    self.output_string.push('*');
                }
                else {
                    self.output_string.push(' ');
                }
            });
            self.output_string.push('\n');
        });
        self.output_string.push_str("\x1b[0m");

        println!("{}", self.output_string);
    }

    fn within_plot(&self, x: f32, y: f32) -> bool {
        self.xrange.contains(x) && self.yrange.contains(y)
    }

    fn to_plotspace(&self, x: f32, y: f32) -> (usize, usize) {
        let xnorm = self.xrange.normalized_coordinate(x);
        let ynorm = 1. - self.yrange.normalized_coordinate(y);

        let xscreen = (xnorm * (self.plot.width as f32)).floor() as usize;
        let yscreen = (ynorm * (self.plot.height as f32)).floor() as usize;

        (xscreen, yscreen)
    }
}
