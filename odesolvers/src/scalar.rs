#![allow(unexpected_cfgs)]

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

pub trait Floating
where
    Self: Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + PartialOrd
        + Sized
        + Default
        + Copy,
{
    fn floatify(value: f64) -> Self;

    fn to_f32(self) -> f32;

    fn to_f64(self) -> f64;
}

#[cfg(FALSE)]
impl Floating for f16 {
    fn floatify(value: f64) -> Self {
        value as f16
    }

    fn to_f32(self) -> f32 {
        self as f32
    }

    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl Floating for f32 {
    fn floatify(value: f64) -> Self {
        value as f32
    }

    fn to_f32(self) -> f32 {
        self
    }

    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl Floating for f64 {
    fn floatify(value: f64) -> Self {
        value
    }

    fn to_f32(self) -> f32 {
        self as f32
    }

    fn to_f64(self) -> f64 {
        self
    }
}

#[cfg(FALSE)]
impl Floating for f128 {
    fn floatify(value: f64) -> Self {
        value as f128
    }

    fn to_f32(self) -> f32 {
        self as f32
    }

    fn to_f64(self) -> f64 {
        self as f64
    }
}
