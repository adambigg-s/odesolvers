use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Default,
{
    pub const fn build(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zeros() -> Self {
        Vec3 { x: T::default(), y: T::default(), z: T::default() }
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T, D> Div<D> for Vec3<T>
where
    T: Div<D, Output = T> + Default,
    D: Clone + Copy,
{
    type Output = Vec3<T>;
    fn div(self, rhs: D) -> Self::Output {
        Vec3::build(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T, D> DivAssign<D> for Vec3<T>
where
    T: DivAssign<D>,
    D: Clone + Copy,
{
    fn div_assign(&mut self, rhs: D) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
