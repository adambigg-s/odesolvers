use crate::integration_shared::DynamicsFunction;
use crate::integration_shared::IntegrationStep;
use crate::integration_shared::Norm;
use crate::integration_shared::State;
use crate::scalar::Floating;

#[derive(Clone, Copy)]
pub struct Integrator<Float, const N: usize> {
    state: State<Float, N>,
    dt: Float,
    ddt: DynamicsFunction<Float, N>,
    time: Float,
}

impl<Float, const N: usize> Integrator<Float, N>
where
    Float: Floating + Default + Copy,
{
    const TOLERANCE: f64 = 1e-8;

    pub fn build(state: [Float; N], delta_time: Float, dynamics: DynamicsFunction<Float, N>) -> Self {
        Integrator { state: State::build(state), dt: delta_time, ddt: dynamics, time: Float::default() }
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
        self.time += self.dt;
        self.state = State::build(self.runge_kutta_4());
        self.state()
    }

    pub fn dynamic_step(&mut self) -> [Float; N] {
        let mut oracle = *self;
        oracle.dt = self.dt * Float::floatify(0.5);
        (0..2).for_each(|_| {
            oracle.step();
        });
        let step = State::build(self.runge_kutta_4());

        let error = (step * Floating::floatify(-1.) + oracle.state).norm();
        if error > Floating::floatify(Self::TOLERANCE) {
            self.dt *= Floating::floatify(0.5);
            return self.dynamic_step();
        }

        self.time += self.dt;
        self.state = step;
        self.dt *= Floating::floatify(2.);

        self.state()
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

    pub fn solve_dynamic_until(&mut self, final_time: Float) -> Vec<[Float; N]> {
        let mut states = Vec::new();
        while self.time < final_time {
            states.push(self.dynamic_step());
        }

        states
    }

    pub fn solve_dynamic_with_time(&mut self, final_time: Float) -> Vec<(Float, [Float; N])> {
        let mut output = Vec::new();
        while self.time < final_time {
            output.push((self.time, self.dynamic_step()));
        }

        output
    }
}

impl<Float, const N: usize> IntegrationStep<[Float; N]> for Integrator<Float, N>
where
    Float: Floating + Default + Copy,
{
    fn runge_kutta_4(&self) -> [Float; N] {
        let k1 = State::build((self.ddt)(&self.state.inner));
        let k2 = State::build((self.ddt)(&(self.state + k1 * (self.dt / Float::floatify(2.))).inner));
        let k3 = State::build((self.ddt)(&(self.state + k2 * (self.dt / Float::floatify(2.))).inner));
        let k4 = State::build((self.ddt)(&(self.state + k3 * self.dt).inner));

        (self.state
            + (k1 + k4) * (self.dt / Float::floatify(6.))
            + (k2 + k3) * (self.dt / Float::floatify(3.)))
        .values()
    }
}
