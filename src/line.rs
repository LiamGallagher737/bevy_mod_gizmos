use bevy::prelude::*;

pub struct Line {
    pub(crate) points: Vec<Vec3>,
    pub(crate) color: Color,
}

impl Line {
    pub fn new(points: Vec<Vec3>, color: Color) -> Self {
        Self { points, color }
    }
}
