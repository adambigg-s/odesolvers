pub mod buffer;
pub mod vec3;

mod scalar;

use std::ops::Add;
use std::ops::Mul;

use scalar::Float;

pub trait DynamicalSystem {
    fn derivative(&self) -> Self;
}

trait IntegrationStep<S> {
    fn rk4(&mut self) -> S;
}

pub struct Integrator<S, T> {
    state: S,
    delta_time: T,
    curr_time: T,
}

impl<S, T> Integrator<S, T>
where
    S: DynamicalSystem + Mul<T, Output = S> + Add<S, Output = S> + Copy,
    T: Float + Default,
{
    pub fn build(integrand: S, dt: T) -> Self {
        Integrator { state: integrand, delta_time: dt, curr_time: T::default() }
    }

    pub const fn state(&self) -> S {
        self.state
    }

    pub const fn delta_time(&self) -> T {
        self.delta_time
    }

    pub const fn curr_time(&self) -> T {
        self.curr_time
    }

    pub fn step(&mut self) -> S {
        self.curr_time = self.curr_time + self.delta_time;
        self.rk4()
    }

    pub fn solve_until(&mut self, final_time: T) -> Vec<S> {
        let mut states = Vec::new();
        while self.curr_time < final_time {
            states.push(self.step());
        }

        states
    }

    pub fn solve_with_time(&mut self, final_time: T) -> Vec<(T, S)> {
        let mut output = Vec::new();
        while self.curr_time < final_time {
            output.push((self.curr_time, self.step()));
        }

        output
    }
}

impl<S, T> IntegrationStep<S> for Integrator<S, T>
where
    S: DynamicalSystem + Mul<T, Output = S> + Add<S, Output = S> + Copy,
    T: Float,
{
    fn rk4(&mut self) -> S {
        let k1 = self.state.derivative();
        let k2 = (self.state + k1 * (self.delta_time / T::float(2.))).derivative();
        let k3 = (self.state + k2 * (self.delta_time / T::float(2.))).derivative();
        let k4 = (self.state + k3 * self.delta_time).derivative();

        self.state = self.state
            + (k1 + k4) * (self.delta_time / T::float(6.))
            + (k2 + k3) * (self.delta_time / T::float(3.));

        self.state
    }
}
