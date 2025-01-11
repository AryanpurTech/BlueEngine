use std::ops::{Add, Mul, Sub, SubAssign};

use bytemuck::{Pod, Zeroable};

/// 3D position
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Zeroable)]
pub struct Position3D {
    /// X coordinate in 3D space
    pub x: f32,
    /// Y coordinate in 3D space
    pub y: f32,
    /// Z coordinate in 3D space
    pub z: f32,
}

impl Position3D {
    /// Create a new 3D position with the given coordinates
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

unsafe impl Send for Position3D {}
unsafe impl Sync for Position3D {}

unsafe impl Pod for Position3D {}

impl Add for Position3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Position3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Position3D {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f32> for Position3D {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl From<[f32; 3]> for Position3D {
    fn from(pos: [f32; 3]) -> Self {
        Self {
            x: pos[0],
            y: pos[1],
            z: pos[2],
        }
    }
}

impl From<Position3D> for [f32; 3] {
    fn from(pos: Position3D) -> Self {
        [pos.x, pos.y, pos.z]
    }
}

impl From<(f32, f32, f32)> for Position3D {
    fn from(pos: (f32, f32, f32)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            z: pos.2,
        }
    }
}

impl From<Position3D> for (f32, f32, f32) {
    fn from(pos: Position3D) -> Self {
        (pos.x, pos.y, pos.z)
    }
}

impl From<nalgebra_glm::Vec3> for Position3D {
    fn from(pos: nalgebra_glm::Vec3) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        }
    }
}

impl From<Position3D> for nalgebra_glm::Vec3 {
    fn from(pos: Position3D) -> Self {
        nalgebra_glm::vec3(pos.x, pos.y, pos.z)
    }
}

/// 2D position
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Position2D {
    /// X coordinate in 2D space
    pub x: f32,
    /// Y coordinate in 2D space
    pub y: f32,
}

impl Position2D {
    /// Create a new 2D position with the given coordinates
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

unsafe impl Send for Position2D {}
unsafe impl Sync for Position2D {}

impl Add for Position2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<[f32; 2]> for Position2D {
    fn from(pos: [f32; 2]) -> Self {
        Self {
            x: pos[0],
            y: pos[1],
        }
    }
}

impl From<Position2D> for [f32; 2] {
    fn from(pos: Position2D) -> Self {
        [pos.x, pos.y]
    }
}

impl From<(f32, f32)> for Position2D {
    fn from(pos: (f32, f32)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
        }
    }
}

impl From<Position2D> for (f32, f32) {
    fn from(pos: Position2D) -> Self {
        (pos.x, pos.y)
    }
}

impl From<nalgebra_glm::Vec2> for Position2D {
    fn from(pos: nalgebra_glm::Vec2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
        }
    }
}

impl From<Position2D> for nalgebra_glm::Vec2 {
    fn from(pos: Position2D) -> Self {
        nalgebra_glm::vec2(pos.x, pos.y)
    }
}