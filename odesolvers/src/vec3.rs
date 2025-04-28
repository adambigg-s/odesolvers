#[derive(Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub const fn build(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}
