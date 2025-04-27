pub mod buffer;
mod scalar;

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;

use scalar::Float;

pub trait DynamicalSystem {
    fn derivative(&self) -> Self;
}

pub struct RungeKutta4<State, Time> {
    pub state: State,
    delta_time: Time,
}

impl<State, Time> RungeKutta4<State, Time>
where
    State: DynamicalSystem + Mul<Time, Output = State> + Add<State, Output = State> + Copy,
    Time: Float + Div<Time, Output = Time> + Copy,
{
    pub fn build(integrand: State, dt: Time) -> Self {
        RungeKutta4 { state: integrand, delta_time: dt }
    }

    pub fn step(&mut self) -> State {
        let k1 = self.state.derivative();
        let k2 = (self.state + k1 * (self.delta_time / Time::float(2.))).derivative();
        let k3 = (self.state + k2 * (self.delta_time / Time::float(2.))).derivative();
        let k4 = (self.state + k3 * self.delta_time).derivative();

        self.state = self.state
            + (k1 + k4) * (self.delta_time / Time::float(6.))
            + (k2 + k3) * (self.delta_time / Time::float(3.));
        self.state
    }
}
