use std::ops::Add;
use std::ops::Mul;

use odesolvers::buffer::Buffer;
use odesolvers::*;

const G: f32 = 9.81;
const M: f32 = 10.;

const PLOT_WIDTH: usize = 200;
const PLOT_HEIGHT: usize = 50;

fn main() {
    println!("\x1b[2J");

    let dt = 0.001;
    let final_time = 100.;
    let initial = State::build(20., 0., 0., 0.5);
    let mut integrator = Integrator::build(initial, dt);
    let mut buffer = Buffer::build(PLOT_WIDTH, PLOT_HEIGHT);

    let mut states = Vec::new();
    loop {
        if integrator.curr_time() > final_time {
            break;
        }

        states.push(integrator.step());

        let (minx, maxx) = states
            .iter()
            .map(|v| v.x)
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), val| (min.min(val), max.max(val)));
        let (miny, maxy) = states
            .iter()
            .map(|v| v.y)
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), val| (min.min(val), max.max(val)));

        if states.len() % 5 != 0 {
            continue;
        }

        states.iter().for_each(|state| {
            let x_norm = (state.x - minx) / (maxx - minx);
            let y_norm = (state.y - miny) / (maxy - miny);

            let xp = (x_norm * buffer.width() * 2.) as usize;
            let yp = (y_norm * buffer.height() * 4.) as usize;

            buffer.set(xp, yp);
        });


        print!("\x1b[0H");

        buffer.render();
        buffer.clear();

        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    println!("lorenz attractor example");
}

#[derive(Clone, Copy)]
pub struct State {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl DynamicalSystem for State {
    fn derivative(&self) -> Self {
        let r_sq = self.x * self.x + self.y * self.y;
        let r = r_sq.sqrt();
        let force_mag = -G * M / r_sq;

        let fx = force_mag * self.x / r;
        let fy = force_mag * self.y / r;

        State::build(self.vx, self.vy, fx, fy)
    }
}

impl State {
    pub fn build(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        State { x, y, vx, vy }
    }
}

impl Add for State {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        State::build(self.x + rhs.x, self.y + rhs.y, self.vx + rhs.vx, self.vy + rhs.vy)
    }
}

impl Mul<f32> for State {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        State::build(self.x * rhs, self.y * rhs, self.vx * rhs, self.vy * rhs)
    }
}
