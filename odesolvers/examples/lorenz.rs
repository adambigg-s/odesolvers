use std::ops::Add;
use std::ops::Mul;

use odesolvers::buffer::Buffer;
use odesolvers::*;

const SIGMA: f32 = 10.;
const RHO: f32 = 28.;
const BETA: f32 = 8. / 3.;

const PLOT_WIDTH: usize = 300;
const PLOT_HEIGHT: usize = 100;

fn main() {
    println!("lorenz attractor example");

    let dt = 0.01;
    let final_time = 60.;
    let lorentz = Vec3::build(10., -2., 1.);
    let mut integrator = Integrator::build(lorentz, dt);
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
}

impl DynamicalSystem for Vec3 {
    fn derivative(&self) -> Self {
        Vec3::build(
            SIGMA * (self.y - self.x),
            self.x * (RHO - self.z) - self.y,
            self.x * self.y - BETA * self.z,
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn build(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::build(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::build(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
