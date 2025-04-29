use crate::integration_shared::IntegrationStep;
use crate::integration_shared::State;
use crate::scalar::Floating;

pub struct Integrator<Float, Dynamics, const N: usize> {
    state: State<Float, N>,
    dt: Float,
    time: Float,
    ddt: Dynamics,
}

impl<Float, Dynamics, const N: usize> Integrator<Float, Dynamics, N>
where
    Float: Floating + Default,
    Dynamics: Fn(&[Float; N]) -> [Float; N],
{
    pub fn build(state: [Float; N], delta_time: Float, dynamics_function: Dynamics) -> Self {
        Integrator {
            state: State::build(state),
            dt: delta_time,
            time: Float::default(),
            ddt: dynamics_function,
        }
    }

    pub const fn state(&self) -> [Float; N] {
        self.state.values()
    }

    pub const fn delta_time(&self) -> Float {
        self.dt
    }

    pub const fn curr_time(&self) -> Float {
        self.time
    }

    pub fn step(&mut self) -> [Float; N] {
        self.time = self.time + self.dt;
        self.rk4()
    }

    pub fn solve_until(&mut self, final_time: Float) -> Vec<[Float; N]> {
        let mut states = Vec::new();
        while self.time < final_time {
            states.push(self.step());
        }

        states
    }

    pub fn solve_with_time(&mut self, final_time: Float) -> Vec<(Float, [Float; N])> {
        let mut output = Vec::new();
        while self.time < final_time {
            output.push((self.time, self.step()));
        }

        output
    }
}

impl<Float, Dynamics, const N: usize> IntegrationStep<[Float; N]> for Integrator<Float, Dynamics, N>
where
    Float: Floating + Default + Copy,
    Dynamics: Fn(&[Float; N]) -> [Float; N],
{
    fn rk4(&mut self) -> [Float; N] {
        let k1 = State::build((self.ddt)(&self.state.inner));
        let k2 = State::build((self.ddt)(&(self.state + k1 * (self.dt / Float::floatify(2.))).inner));
        let k3 = State::build((self.ddt)(&(self.state + k2 * (self.dt / Float::floatify(2.))).inner));
        let k4 = State::build((self.ddt)(&(self.state + k3 * self.dt).inner));

        self.state = self.state
            + (k1 + k4) * (self.dt / Float::floatify(6.))
            + (k2 + k3) * (self.dt / Float::floatify(3.));

        self.state()
    }
}
