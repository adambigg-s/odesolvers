use std::ops::Add;
use std::ops::Mul;

use odesolvers::buffer::Buffer;
use odesolvers::*;

const SIGMA: f32 = 10.;
const RHO: f32 = 28.;
const BETA: f32 = 8. / 3.;

const PLOT_WIDTH: usize = 220;
const PLOT_HEIGHT: usize = 70;

fn main() {
    println!("\x1b[2J");

    let dt = 0.01;
    let final_time = 100.;
    let lorentz = LorenzSystem::build(1., -1., 0.5);
    let mut integrator = Integrator::build(lorentz, dt);
    let mut buffer = Buffer::build(PLOT_WIDTH, PLOT_HEIGHT);

    let mut states = Vec::new();
    while integrator.curr_time() < final_time {
        states.push(integrator.step());

        buffer.plot_linstrip_2d(states.iter().map(|state| state.x), states.iter().map(|state| state.y));

        buffer.render();
        buffer.clear();
    }

    println!("lorenz attractor example");
}

impl DynamicalSystem for LorenzSystem {
    fn derivative(&self) -> Self {
        LorenzSystem::build(
            SIGMA * (self.y - self.x),
            self.x * (RHO - self.z) - self.y,
            self.x * self.y - BETA * self.z,
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LorenzSystem {
    x: f32,
    y: f32,
    z: f32,
}

impl LorenzSystem {
    pub fn build(x: f32, y: f32, z: f32) -> Self {
        LorenzSystem { x, y, z }
    }
}

impl Add for LorenzSystem {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        LorenzSystem::build(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f32> for LorenzSystem {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        LorenzSystem::build(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
