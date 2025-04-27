use std::ops::Add;
use std::ops::Mul;

pub trait DynamicalSystem {
    fn derivative(&self) -> Self;
}

pub struct RungeKutta4<State> {
    pub state: State,
}

impl<State> RungeKutta4<State>
where
    State: DynamicalSystem + Mul<f32, Output = State> + Add<State, Output = State> + Copy,
{
    pub fn build(integrand: State) -> Self {
        RungeKutta4 { state: integrand }
    }

    pub fn step(&mut self, dt: f32) -> State {
        let k1 = self.state.derivative();
        let k2 = (self.state + k1 * (dt / 2.)).derivative();
        let k3 = (self.state + k2 * (dt / 2.)).derivative();
        let k4 = (self.state + k3 * dt).derivative();

        self.state = self.state + (k1 + k4) * (dt / 6.) + (k2 + k3) * (dt / 3.);
        self.state
    }
}
