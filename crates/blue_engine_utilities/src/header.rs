use blue_engine::{Vector3, Vector4};
// ========== For keyframe animation =============== //
#[cfg(feature = "animation")]
use keyframe_derive::CanTween;

#[cfg(feature = "animation")]
pub use keyframe::functions as animation_functions;

#[cfg(feature = "animation")]
#[derive(CanTween, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[cfg(feature = "animation")]
impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
#[cfg(feature = "animation")]
impl Default for Point3 {
    fn default() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }
}
#[cfg(feature = "animation")]
impl From<(f32, f32, f32)> for Point3 {
    fn from(data: (f32, f32, f32)) -> Self {
        Self {
            x: data.0,
            y: data.1,
            z: data.2,
        }
    }
}

#[cfg(feature = "animation")]
/// A frame for animation, has position, rotation, and size
#[derive(CanTween, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AnimationKeyframe {
    /// position to set, default is 0f32 for xyz
    pub position: Point3,
    /// rotatioon to set (uses Euler angles), default is 0f32 for xyz
    pub rotation: Point3,
    /// resize to set, default is 100f32 for xyz
    pub size: Point3,
}
#[cfg(feature = "animation")]
impl Default for AnimationKeyframe {
    fn default() -> Self {
        Self {
            position: Point3::default(),
            rotation: Point3::default(),
            size: Point3::new(100f32, 100f32, 100f32),
        }
    }
}

pub struct FlyCamera {
    pub camera_right: Vector3,
    pub yaw: f32,
    pub pitch: f32,
    pub last_x: f64,
    pub last_y: f64,

    pub is_focus: bool,
    pub camera_speed: f32,
    pub camera_sensitivity: f32,
    pub timer: std::time::Instant,
    pub last_frame: f32,

    pub test_counter: u32,
}

#[derive(Debug, Clone)]
pub struct LightManager {
    pub ambient_color: Vector4,
    pub ambient_strength: f32,
    pub affected_objects: Vec<String>,
    pub light_objects: std::collections::BTreeMap<String, ([f32; 3], Vector4)>,
}
