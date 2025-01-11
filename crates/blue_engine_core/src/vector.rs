use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{
    header::{Vector2, Vector3},
    RotateAmount, RotateAxis,
};
use bytemuck::Pod;

// Constructors
impl Vector3 {
    /// A vector with all components set to 0.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    /// A vector with all components set to 1.
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);
    /// A vector with all components set to 0 except for the x component, which is set to 1.
    pub const UNIT_X: Self = Self::new(1.0, 0.0, 0.0);
    /// A vector with all components set to 0 except for the y component, which is set to 1.
    pub const UNIT_Y: Self = Self::new(0.0, 1.0, 0.0);
    /// A vector with all components set to 0 except for the z component, which is set to 1.
    pub const UNIT_Z: Self = Self::new(0.0, 0.0, 1.0);

    /// Create a new 3D position with the given coordinates
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    /// Returns a vector with all components set to 0 except for the x component, which is set to 1.
    pub const fn x_axis() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    /// Returns a vector with all components set to 0 except for the y component, which is set to 1.
    pub const fn y_axis() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
    /// Returns a vector with all components set to 0 except for the z component, which is set to 1.
    pub const fn z_axis() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
    /// Returns a 2D vector with the x and y coordinates of the 3D vector
    pub const fn xy(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
    /// Returns a 2D vector with the x and z coordinates of the 3D vector
    pub const fn xz(&self) -> Vector2 {
        Vector2::new(self.x, self.z)
    }
    /// Returns a 2D vector with the y and z coordinates of the 3D vector
    pub const fn yz(&self) -> Vector2 {
        Vector2::new(self.y, self.z)
    }
}

// Methods
impl Vector3 {
    /// Returns a new vector with all components in absolute values (i.e. positive).
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
    /// Returns the unsigned minimum angle to the given vector, in radians.
    pub fn angle_to(&self, to: Self) -> f32 {
        let dot = self.dot(to);
        let len = self.length() * to.length();
        (dot / len).acos()
    }
    /// Returns the vector "bounced off" from a plane defined by the given normal ``n``.
    ///
    ///> Note: bounce performs the operation that most engines and frameworks call ``reflect()``.
    pub fn bounce(&self, n: Self) -> Self {
        *self - n * 2.0 * self.dot(n)
    }
    /// Returns a new vector with all components rounded up (towards positive infinity).
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }
    /// Returns a new vector with all components clamped between the components of ``min`` and ``max``
    pub fn clamp(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
        }
    }
    /// Returns a new vector with all components clamped between ``min`` and ``max``
    pub fn clampf(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
        }
    }
    /// Returns the cross product of this vector and ``with``.
    ///
    /// This returns a vector perpendicular to both ``self`` and ``with``, which would be the normal vector of the plane defined by the two vectors.
    /// As there are two such vectors, in opposite directions, this method returns the vector defined by a right-handed coordinate system.
    /// If the two vectors are parallel this returns an empty vector, making it useful for testing if two vectors are parallel.
    pub const fn cross(&self, with: Self) -> Self {
        Self {
            x: self.y * with.z - self.z * with.y,
            y: self.z * with.x - self.x * with.z,
            z: self.x * with.y - self.y * with.x,
        }
    }
    /// Returns the normalized vector pointing from this vector to ``to``. This is equivalent to using ``(b - a).normalized()``.
    pub fn direction_to(&self, to: Self) -> Self {
        (to - *self).normalize()
    }
    /// Returns the squared distance between this vector and ``to``.
    ///
    /// This method runs faster than [``Vector3::distance_to``], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub fn distance_squared_to(&self, to: Self) -> f32 {
        (*self - to).length_squared()
    }
    /// Returns the distance between this vector and ``to``.
    pub fn distance_to(&self, to: Self) -> f32 {
        (*self - to).length()
    }
    /// Returns the dot product of this vector and ``with``. This can be used to compare the angle between two vectors. For example, this can be used to determine whether an enemy is facing the player.
    ///
    /// The dot product will be ``0`` for a right angle (90 degrees), greater than 0 for angles narrower than 90 degrees and lower than 0 for angles wider than 90 degrees.
    ///
    /// When using unit (normalized) vectors, the result will always be between ``-1.0`` (180 degree angle) when the vectors are facing opposite directions, and ``1.0`` (0 degree angle) when the vectors are aligned.
    ///
    ///> Note: ``a.dot(b)`` is equivalent to ``b.dot(a)``.
    pub fn dot(&self, with: Self) -> f32 {
        self.x * with.x + self.y * with.y + self.z * with.z
    }
    /// Returns a new vector with all components rounded down (towards negative infinity).
    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
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
    pub const fn inverse(&self) -> Self {
        Self {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
        }
    }
    /// Returns true if the vector is normalized, i.e. its length is approximately equal to 1.
    #[must_use]
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
    pub const fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    /// Returns the result of the linear interpolation between this vector and ``to`` by amount ``weight``.
    /// ``weight`` is on the range of ``0.0`` to ``1.0``, representing the amount of interpolation.
    pub fn lerp(&self, to: Self, weight: f32) -> Self {
        *self * (1.0 - weight) + to * weight
    }
    /// Returns the vector with a maximum length by limiting its length to ``length``.
    pub fn limit_length(&self, max_length: f32) -> Self {
        let length = self.length();
        if length > max_length {
            *self * (max_length / length)
        } else {
            *self
        }
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
    /// Normalize the vector (make the length of the vector 1)
    pub fn normalize(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
    /// Returns a new vector with all components rounded to the nearest integer, with halfway cases rounded away from zero.
    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
        }
    }
    /// Rotates the vector around the given axis by the given angle.    
    pub fn rotate(&self, axis: RotateAxis, angle: RotateAmount) -> Self {
        let angle = match angle {
            RotateAmount::Degrees(d) => d.to_radians(),
            RotateAmount::Radians(r) => r,
        };
        match axis {
            RotateAxis::X => {
                let cos = angle.cos();
                let sin = angle.sin();
                Self {
                    x: self.x,
                    y: self.y * cos - self.z * sin,
                    z: self.y * sin + self.z * cos,
                }
            }
            RotateAxis::Y => {
                let cos = angle.cos();
                let sin = angle.sin();
                Self {
                    x: self.x * cos + self.z * sin,
                    y: self.y,
                    z: -self.x * sin + self.z * cos,
                }
            }
            RotateAxis::Z => {
                let cos = angle.cos();
                let sin = angle.sin();
                Self {
                    x: self.x * cos - self.y * sin,
                    y: self.x * sin + self.y * cos,
                    z: self.z,
                }
            }
        }
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

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
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

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Constructors
impl Vector2 {
    /// A vector with all components set to 0.
    pub const ZERO: Self = Self::new(0.0, 0.0);
    /// A vector with all components set to 1.
    pub const ONE: Self = Self::new(1.0, 1.0);
    /// A vector with all components set to 0 except for the x component, which is set to 1.
    pub const UNIT_X: Self = Self::new(1.0, 0.0);
    /// A vector with all components set to 0 except for the y component, which is set to 1.
    pub const UNIT_Y: Self = Self::new(0.0, 1.0);

    /// Create a new 2D position with the given coordinates
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    /// Returns a vector with all components set to 0 except for the x component, which is set to 1.
    pub const fn x_axis() -> Self {
        Self::new(1.0, 0.0)
    }
    /// Returns a vector with all components set to 0 except for the y component, which is set to 1.
    pub const fn y_axis() -> Self {
        Self::new(0.0, 1.0)
    }
}

// Methods
impl Vector2 {
    /// Returns a new vector with all components in absolute values (i.e. positive).
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
    /// Returns the unsigned minimum angle to the given vector, in radians.
    pub fn angle_to(&self, to: Self) -> f32 {
        let dot = self.dot(to);
        let len = self.length() * to.length();
        (dot / len).acos()
    }
    /// Returns the vector "bounced off" from a plane defined by the given normal ``n``.
    ///
    ///> Note: bounce performs the operation that most engines and frameworks call ``reflect()``.
    pub fn bounce(&self, n: Self) -> Self {
        *self - n * 2.0 * self.dot(n)
    }
    /// Returns a new vector with all components rounded up (towards positive infinity).
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }
    /// Returns a new vector with all components clamped between the components of ``min`` and ``max``
    pub fn clamp(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }
    /// Returns a new vector with all components clamped between ``min`` and ``max``
    pub fn clampf(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }
    /// Returns the cross product of this vector and `with`.
    ///
    /// This returns a scalar representing the magnitude of the vector perpendicular to the plane defined by the two vectors.
    /// If the two vectors are parallel, this returns `0.0`, making it useful for testing if two vectors are parallel.
    pub const fn cross(&self, with: Self) -> f32 {
        self.x * with.y - self.y * with.x
    }
    /// Returns the normalized vector pointing from this vector to ``to``. This is equivalent to using ``(b - a).normalized()``.
    pub fn direction_to(&self, to: Self) -> Self {
        (to - *self).normalize()
    }
    /// Returns the squared distance between this vector and ``to``.
    ///
    /// This method runs faster than [``Vector2::distance_to``], so prefer it if you need to compare vectors or need the squared distance for some formula.
    pub fn distance_squared_to(&self, to: Self) -> f32 {
        (*self - to).length_squared()
    }
    /// Returns the distance between this vector and ``to``.
    pub fn distance_to(&self, to: Self) -> f32 {
        (*self - to).length()
    }
    /// Returns the dot product of this vector and ``with``. This can be used to compare the angle between two vectors. For example, this can be used to determine whether an enemy is facing the player.
    ///
    /// The dot product will be ``0`` for a right angle (90 degrees), greater than 0 for angles narrower than 90 degrees and lower than 0 for angles wider than 90 degrees.
    ///
    /// When using unit (normalized) vectors, the result will always be between ``-1.0`` (180 degree angle) when the vectors are facing opposite directions, and ``1.0`` (0 degree angle) when the vectors are aligned.
    ///
    ///> Note: ``a.dot(b)`` is equivalent to ``b.dot(a)``.
    pub fn dot(&self, with: Self) -> f32 {
        self.x * with.x + self.y * with.y
    }
    /// Returns a new vector with all components rounded down (towards negative infinity).
    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
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
    pub const fn inverse(&self) -> Self {
        Self {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
        }
    }
    /// Returns true if the vector is normalized, i.e. its length is approximately equal to 1.
    #[must_use]
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
    pub const fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    /// Returns the result of the linear interpolation between this vector and ``to`` by amount ``weight``.
    /// ``weight`` is on the range of ``0.0`` to ``1.0``, representing the amount of interpolation.
    pub fn lerp(&self, to: Self, weight: f32) -> Self {
        *self * (1.0 - weight) + to * weight
    }
    /// Returns the vector with a maximum length by limiting its length to ``length``.
    pub fn limit_length(&self, max_length: f32) -> Self {
        let length = self.length();
        if length > max_length {
            *self * (max_length / length)
        } else {
            *self
        }
    }
    /// Returns the component-wise minimum of ``self`` and ``with``, equivalent to:
    /// ```rs
    /// Vector2 {
    ///    x: self.x.max(with.x),
    ///    y: self.y.max(with.y),
    /// }
    /// ```
    pub fn max(&self, with: Vector3) -> Self {
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
    pub fn min(&self, with: Vector3) -> Self {
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
    /// Normalize the vector (make the length of the vector 1)
    pub fn normalize(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y).sqrt();
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }
    /// Returns a new vector with all components rounded to the nearest integer, with halfway cases rounded away from zero.
    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }
    /// Rotates the vector around the given axis by the given angle.
    pub fn rotate(&self, axis: RotateAxis, angle: RotateAmount) -> Self {
        // axis might be unused
        let angle = match angle {
            RotateAmount::Degrees(d) => d.to_radians(),
            RotateAmount::Radians(r) => r,
        };

        let (sin, cos) = angle.sin_cos();

        match axis {
            RotateAxis::Z => Self {
                x: self.x * cos - self.y * sin,
                y: self.x * sin + self.y * cos,
            },
            _ => *self, // For Vector2, only Z-axis rotation makes sense
        }
    }
}

unsafe impl Send for Vector2 {}
unsafe impl Sync for Vector2 {}

unsafe impl Pod for Vector2 {}

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

impl Index<usize> for Vector2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
