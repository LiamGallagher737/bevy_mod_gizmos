use bevy::prelude::*;

/// A pixel wide lline
pub struct Line {
    pub(crate) points: Vec<Vec3>,
    pub(crate) color: Color,
}

impl Line {
    /// Line constructor
    pub fn new(points: Vec<Vec3>, color: Color) -> Self {
        Self { points, color }
    }
}
