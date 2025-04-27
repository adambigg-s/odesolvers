#![allow(unexpected_cfgs)]

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

pub trait Float
where
    Self: Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + PartialOrd
        + Sized
        + Copy,
{
    fn float(value: f64) -> Self;
}

#[cfg(FALSE)]
impl Float for f16 {
    fn float(value: f64) -> Self {
        value as f16
    }
}

impl Float for f32 {
    fn float(value: f64) -> Self {
        value as f32
    }
}

impl Float for f64 {
    fn float(value: f64) -> Self {
        value
    }
}

#[cfg(FALSE)]
impl Float for f128 {
    fn float(value: f64) -> Self {
        value as f128
    }
}
