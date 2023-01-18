use crate::interactions::GizmoInteractions;
use bevy::prelude::*;

#[derive(Debug)]
pub struct Gizmo {
    pub(crate) translation: Vec3,
    pub(crate) size: f32,
    pub(crate) color: Color,
    pub(crate) interactions: GizmoInteractions,
}

impl Gizmo {
    pub fn new(translation: Vec3, size: f32, color: Color) -> Self {
        Self {
            translation,
            size,
            color,
            interactions: Default::default(),
        }
    }
}

impl From<Vec3> for Gizmo {
    fn from(value: Vec3) -> Self {
        Self::new(value, 1.0, Color::RED)
    }
}

impl From<[f32; 3]> for Gizmo {
    fn from(value: [f32; 3]) -> Self {
        Self::new(value.into(), 1.0, Color::RED)
    }
}

impl From<(f32, f32, f32)> for Gizmo {
    fn from(value: (f32, f32, f32)) -> Self {
        Self::new(value.into(), 1.0, Color::RED)
    }
}

impl From<(Vec3, f32)> for Gizmo {
    fn from(value: (Vec3, f32)) -> Self {
        Self::new(value.0, value.1, Color::RED)
    }
}

impl From<(Vec3, Color)> for Gizmo {
    fn from(value: (Vec3, Color)) -> Self {
        Self::new(value.0, 1.0, value.1)
    }
}

impl From<(Vec3, f32, Color)> for Gizmo {
    fn from(value: (Vec3, f32, Color)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}
