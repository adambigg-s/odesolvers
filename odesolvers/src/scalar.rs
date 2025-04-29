#![allow(unexpected_cfgs)]

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

pub trait Floating
where
    Self: Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + PartialOrd
        + Sized
        + Copy,
{
    fn floatify(value: f64) -> Self;
}

#[cfg(FALSE)]
impl Floating for f16 {
    fn floatify(value: f64) -> Self {
        value as f16
    }
}

impl Floating for f32 {
    fn floatify(value: f64) -> Self {
        value as f32
    }
}

impl Floating for f64 {
    fn floatify(value: f64) -> Self {
        value
    }
}

#[cfg(FALSE)]
impl Floating for f128 {
    fn floatify(value: f64) -> Self {
        value as f128
    }
}
