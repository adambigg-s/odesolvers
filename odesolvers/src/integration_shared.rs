use std::ops::Add;
use std::ops::Mul;

pub trait IntegrationStep<State> {
    fn rk4(&mut self) -> State;
}

#[derive(Clone, Copy)]
pub struct State<Float, const N: usize> {
    pub inner: [Float; N],
}

impl<Float, const N: usize> Add for State<Float, N>
where
    Float: Add<Output = Float> + Default + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = [Float::default(); N];
        (0..N).for_each(|idx| {
            result[idx] = self.inner[idx] + rhs.inner[idx];
        });

        State::build(result)
    }
}

impl<Float, const N: usize> Mul<Float> for State<Float, N>
where
    Float: Mul<Float, Output = Float> + Default + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Float) -> Self::Output {
        let mut result = [Float::default(); N];
        (0..N).for_each(|idx| {
            result[idx] = self.inner[idx] * rhs;
        });

        State::build(result)
    }
}

impl<Float, const N: usize> State<Float, N>
where
    Float: Copy,
{
    pub const fn build(inner: [Float; N]) -> Self {
        State { inner }
    }

    pub const fn values(&self) -> [Float; N] {
        self.inner
    }
}
