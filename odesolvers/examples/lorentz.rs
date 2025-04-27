use std::ops::Add;
use std::ops::Mul;

use odesolvers::*;

const SIGMA: f32 = 10.;
const RHO: f32 = 28.;
const BETA: f32 = 8. / 3.;

fn main() {
    println!("lorentz attractor example");

    let dt = 0.01;
    let steps = 1000;

    let lorentz = Vec3::build(1., 0., 0.);
    let mut integrator = RungeKutta4::build(lorentz);

    let mut states = Vec::with_capacity(steps);
    (0..steps).for_each(|_| {
        states.push(integrator.step(dt));
    });

    println!("states: {:?}", states);
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
