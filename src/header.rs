// ========== For keyframe animation =============== //
use keyframe_derive::CanTween;

#[derive(CanTween, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
impl Default for Point3 {
    fn default() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }
}
impl From<(f32, f32, f32)> for Point3 {
    fn from(data: (f32, f32, f32)) -> Self {
        Self {
            x: data.0,
            y: data.1,
            z: data.2,
        }
    }
}

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
impl Default for AnimationKeyframe {
    fn default() -> Self {
        Self {
            position: Point3::default(),
            rotation: Point3::default(),
            size: Point3::new(100f32,100f32,100f32),
        }
    }
}