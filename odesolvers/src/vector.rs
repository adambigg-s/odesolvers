use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

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

    pub fn cast<D>(self) -> Vec3<D>
    where
        T: Into<D>,
        D: Default,
    {
        Vec3::build(self.x.into(), self.y.into(), self.z.into())
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T> + Default,
{
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::build(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T> + Default,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::build(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T, D> Mul<D> for Vec3<T>
where
    T: Mul<D, Output = T> + Default,
    D: Clone + Copy,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: D) -> Self::Output {
        Vec3::build(self.x * rhs, self.y * rhs, self.z * rhs)
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

impl<T> SubAssign for Vec3<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T, D> MulAssign<D> for Vec3<T>
where
    T: MulAssign<D>,
    D: Clone + Copy,
{
    fn mul_assign(&mut self, rhs: D) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
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
