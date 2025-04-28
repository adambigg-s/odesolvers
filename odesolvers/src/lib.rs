pub mod buffer;
pub mod vec3;

mod scalar;

use std::ops::Add;
use std::ops::Mul;

use scalar::Float;

pub trait DynamicalSystem {
    fn derivative(&self) -> Self;
}

trait IntegrationStep<St> {
    fn rk4(&mut self) -> St;
}

pub struct Integrator<St, Ti> {
    state: St,
    delta_time: Ti,
    curr_time: Ti,
}

impl<St, Ti> Integrator<St, Ti>
where
    St: DynamicalSystem + Mul<Ti, Output = St> + Add<St, Output = St> + Copy,
    Ti: Float + Default,
{
    pub fn build(integrand: St, dt: Ti) -> Self {
        Integrator { state: integrand, delta_time: dt, curr_time: Ti::default() }
    }

    pub const fn state(&self) -> St {
        self.state
    }

    pub const fn delta_time(&self) -> Ti {
        self.delta_time
    }

    pub const fn curr_time(&self) -> Ti {
        self.curr_time
    }

    pub fn step(&mut self) -> St {
        self.curr_time = self.curr_time + self.delta_time;
        self.rk4()
    }

    pub fn solve_until(&mut self, final_time: Ti) -> Vec<St> {
        let mut states = Vec::new();
        while self.curr_time < final_time {
            states.push(self.step());
        }

        states
    }

    pub fn solve_with_time(&mut self, final_time: Ti) -> Vec<(Ti, St)> {
        let mut output = Vec::new();
        while self.curr_time < final_time {
            output.push((self.curr_time, self.step()));
        }

        output
    }
}

impl<St, Ti> IntegrationStep<St> for Integrator<St, Ti>
where
    St: DynamicalSystem + Mul<Ti, Output = St> + Add<St, Output = St> + Copy,
    Ti: Float,
{
    fn rk4(&mut self) -> St {
        let k1 = self.state.derivative();
        let k2 = (self.state + k1 * (self.delta_time / Ti::float(2.))).derivative();
        let k3 = (self.state + k2 * (self.delta_time / Ti::float(2.))).derivative();
        let k4 = (self.state + k3 * self.delta_time).derivative();

        self.state = self.state
            + (k1 + k4) * (self.delta_time / Ti::float(6.))
            + (k2 + k3) * (self.delta_time / Ti::float(3.));

        self.state
    }
}
