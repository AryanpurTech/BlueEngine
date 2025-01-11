use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::header::{Vector2, Vector3};
use bytemuck::Pod;

impl Vector3 {
    /// Create a new 3D position with the given coordinates
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    /// Normalize the vector (make the length of the vector 1)
    pub fn normalize(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
    /// Returns a new vector with all components rounded up (towards positive infinity).
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }
    /// Returns a new vector with all components rounded down (towards negative infinity).
    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }
    /// Returns a 2D vector with the x and y coordinates of the 3D vector
    pub fn xy(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
    /// Returns a 2D vector with the x and z coordinates of the 3D vector
    pub fn xz(&self) -> Vector2 {
        Vector2::new(self.x, self.z)
    }
    /// Returns a 2D vector with the y and z coordinates of the 3D vector
    pub fn yz(&self) -> Vector2 {
        Vector2::new(self.y, self.z)
    }
    /// Returns a new vector with all components clamped between the components of ``min`` and ``max``
    pub fn clamp(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
        }
    }
    /// Returns the inverse of the vector. This is the same as:
    ///```rs
    /// Vector3 {
    ///     x: 1.0 / self.x,
    ///     y: 1.0 / self.y,
    ///     z: 1.0 / self.z
    /// }
    /// ```
    pub fn inverse(&self) -> Self {
        Self {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
        }
    }
    /// Returns true if the vector is normalized, i.e. its length is approximately equal to 1.
    pub fn is_normalized(&self) -> bool {
        (self.x * self.x + self.y * self.y + self.z * self.z - 1.0).abs() < 0.0001
    }
    /// Returns the length (magnitude) of this vector.
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    /// Returns the squared length (squared magnitude) of this vector.
    ///
    /// This method runs faster than [`Vector3::length`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    /// Returns the component-wise minimum of ``self`` and ``with``, equivalent to:
    /// ```rs
    /// Vector3 {
    ///    x: self.x.max(with.x),
    ///    y: self.y.max(with.y),
    ///    z: self.z.max(with.z),
    /// }
    /// ```
    pub fn max(&self, with: Vector3) -> Self {
        Self {
            x: self.x.max(with.x),
            y: self.y.max(with.y),
            z: self.z.max(with.z),
        }
    }
    /// Returns the component-wise maximum of ``self`` and ``with``, equivalent to:
    /// ```rs
    /// Vector3 {
    ///    x: self.x.max(with),
    ///    y: self.y.max(with),
    ///    z: self.z.max(with),
    /// }
    /// ```
    pub fn maxf(&self, with: f32) -> Self {
        Self {
            x: self.x.max(with),
            y: self.y.max(with),
            z: self.z.max(with),
        }
    }
    /// Returns the component-wise minimum of ``self`` and ``with``, equivalent to:
    /// ```rs
    /// Vector3 {
    ///    x: self.x.min(with.x),
    ///    y: self.y.min(with.y),
    ///    z: self.z.min(with.z),
    /// }
    /// ```
    pub fn min(&self, with: Vector3) -> Self {
        Self {
            x: self.x.min(with.x),
            y: self.y.min(with.y),
            z: self.z.min(with.z),
        }
    }
    /// Returns the component-wise minimum of ``self`` and ``with``, equivalent to:
    /// ```rs
    /// Vector3 {
    ///    x: self.x.min(with),
    ///    y: self.y.min(with),
    ///    z: self.z.min(with),
    /// }
    /// ```
    pub fn minf(&self, with: f32) -> Self {
        Self {
            x: self.x.min(with),
            y: self.y.min(with),
            z: self.z.min(with),
        }
    }
    /// Returns a vector with all components set to 0 except for the x component, which is set to 1.
    pub fn x_axis() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    /// Returns a vector with all components set to 0 except for the y component, which is set to 1.
    pub fn y_axis() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
    /// Returns a vector with all components set to 0 except for the z component, which is set to 1.
    pub fn z_axis() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
}

unsafe impl Send for Vector3 {}
unsafe impl Sync for Vector3 {}

unsafe impl Pod for Vector3 {}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f32> for Vector3 {
    type Output = Self;

    fn add(self, scalar: f32) -> Self {
        Self {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<f32> for Vector3 {
    fn add_assign(&mut self, scalar: f32) {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f32> for Vector3 {
    type Output = Self;

    fn sub(self, scalar: f32) -> Self {
        Self {
            x: self.x - scalar,
            y: self.y - scalar,
            z: self.z - scalar,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl SubAssign<f32> for Vector3 {
    fn sub_assign(&mut self, scalar: f32) {
        self.x -= scalar;
        self.y -= scalar;
        self.z -= scalar;
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl DivAssign for Vector3 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl From<[f32; 3]> for Vector3 {
    fn from(pos: [f32; 3]) -> Self {
        Self {
            x: pos[0],
            y: pos[1],
            z: pos[2],
        }
    }
}

impl From<Vector3> for [f32; 3] {
    fn from(pos: Vector3) -> Self {
        [pos.x, pos.y, pos.z]
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(pos: (f32, f32, f32)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            z: pos.2,
        }
    }
}

impl From<Vector3> for (f32, f32, f32) {
    fn from(pos: Vector3) -> Self {
        (pos.x, pos.y, pos.z)
    }
}

impl From<nalgebra_glm::Vec3> for Vector3 {
    fn from(pos: nalgebra_glm::Vec3) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        }
    }
}

impl From<Vector3> for nalgebra_glm::Vec3 {
    fn from(pos: Vector3) -> Self {
        nalgebra_glm::vec3(pos.x, pos.y, pos.z)
    }
}

impl Vector2 {
    /// Create a new 2D position with the given coordinates
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    /// Normalize the vector
    pub fn normalize(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y).sqrt();
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }
    /// Round the vector to the nearest upper integer
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }
    /// Round the vector to the nearest lower integer
    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }
    /// Returns a new vector with all components clamped between the components of ``min`` and ``max``
    pub fn clamp(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }
    /// Returns the inverse of the vector. This is the same as:
    ///```rs
    /// Vector2 {
    ///     x: 1.0 / self.x,
    ///     y: 1.0 / self.y,
    /// }
    /// ```
    pub fn inverse(&self) -> Self {
        Self {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
        }
    }
    /// Returns true if the vector is normalized, i.e. its length is approximately equal to 1.
    pub fn is_normalized(&self) -> bool {
        (self.x * self.x + self.y * self.y - 1.0).abs() < 0.0001
    }
    /// Returns the length (magnitude) of this vector.
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    /// Returns the squared length (squared magnitude) of this vector.
    ///
    /// This method runs faster than [`Vector2::length`], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    /// Returns the component-wise minimum of ``self`` and ``with``, equivalent to:
    /// ```rs
    /// Vector2 {
    ///    x: self.x.max(with.x),
    ///    y: self.y.max(with.y),
    /// }
    /// ```
    pub fn max(&self, with: Vector2) -> Self {
        Self {
            x: self.x.max(with.x),
            y: self.y.max(with.y),
        }
    }
    /// Returns the component-wise maximum of ``self`` and ``with``, equivalent to:
    /// ```rs
    /// Vector2 {
    ///    x: self.x.max(with),
    ///    y: self.y.max(with),
    /// }
    /// ```
    pub fn maxf(&self, with: f32) -> Self {
        Self {
            x: self.x.max(with),
            y: self.y.max(with),
        }
    }
    /// Returns the component-wise minimum of ``self`` and ``with``, equivalent to:
    /// ```rs
    /// Vector2 {
    ///    x: self.x.min(with.x),
    ///    y: self.y.min(with.y),
    /// }
    /// ```
    pub fn min(&self, with: Vector2) -> Self {
        Self {
            x: self.x.min(with.x),
            y: self.y.min(with.y),
        }
    }
    /// Returns the component-wise minimum of ``self`` and ``with``, equivalent to:
    /// ```rs
    /// Vector2 {
    ///    x: self.x.min(with),
    ///    y: self.y.min(with),
    /// }
    /// ```
    pub fn minf(&self, with: f32) -> Self {
        Self {
            x: self.x.min(with),
            y: self.y.min(with),
        }
    }
    /// Returns a vector with all components set to 0 except for the x component, which is set to 1.
    pub fn x_axis() -> Self {
        Self::new(1.0, 0.0)
    }
    /// Returns a vector with all components set to 0 except for the y component, which is set to 1.
    pub fn y_axis() -> Self {
        Self::new(0.0, 1.0)
    }
}

unsafe impl Send for Vector2 {}
unsafe impl Sync for Vector2 {}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<f32> for Vector2 {
    type Output = Self;

    fn add(self, scalar: f32) -> Self {
        Self {
            x: self.x + scalar,
            y: self.y + scalar,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, scalar: f32) {
        self.x += scalar;
        self.y += scalar;
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<f32> for Vector2 {
    type Output = Self;

    fn sub(self, scalar: f32) -> Self {
        Self {
            x: self.x - scalar,
            y: self.y - scalar,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, scalar: f32) {
        self.x -= scalar;
        self.y -= scalar;
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign for Vector2 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl From<[f32; 2]> for Vector2 {
    fn from(pos: [f32; 2]) -> Self {
        Self {
            x: pos[0],
            y: pos[1],
        }
    }
}

impl From<Vector2> for [f32; 2] {
    fn from(pos: Vector2) -> Self {
        [pos.x, pos.y]
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(pos: (f32, f32)) -> Self {
        Self { x: pos.0, y: pos.1 }
    }
}

impl From<Vector2> for (f32, f32) {
    fn from(pos: Vector2) -> Self {
        (pos.x, pos.y)
    }
}

impl From<nalgebra_glm::Vec2> for Vector2 {
    fn from(pos: nalgebra_glm::Vec2) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

impl From<Vector2> for nalgebra_glm::Vec2 {
    fn from(pos: Vector2) -> Self {
        nalgebra_glm::vec2(pos.x, pos.y)
    }
}
