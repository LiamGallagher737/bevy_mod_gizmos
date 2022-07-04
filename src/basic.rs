//! This contains the basic gizmo tools for drawing gizmos and lines

use bevy::math::Vec3;

use crate::*;

pub use crate::gizmo::Gizmo;

/// Draws a gizmo for a signle frame
/// # Arguments
/// * `gizmo` - The gizmo to spawn, this can be any struct that implements [`Gizmo`]
pub fn draw_gizmo(gizmo: Gizmo) {
    if let Ok(mut gizmo_buffer) = GIZMO_BUFFER.write() {
        gizmo_buffer.push(gizmo);
    }
}

/// Draws multiple gizmos for a single frame
/// # Arguments
/// * `gizmos` - The gizmos to spawn, this is a [`Vec`] of any struct that implements [`Gizmo`]
/// * `line` - Whether or not you want to draw a line between the gizmos
pub fn draw_gizmos(mut gizmos: Vec<Gizmo>, line: bool) {
    if gizmos.is_empty() {
        return;
    }
    if line {
        draw_line(
            gizmos.iter().map(|g| g.transform.translation).collect(),
            gizmos[0].color,
        );
    }
    if let Ok(mut gizmo_buffer) = GIZMO_BUFFER.write() {
        while let Some(gizmo) = gizmos.pop() {
            gizmo_buffer.push(gizmo);
        }
    }
}

/// Draws a lines from a list of points
/// # Arguments
/// * `points` - A [`Vec`] of [`Vec3`] holding the lne points
/// * `color` - The color you would lke the line to be
pub fn draw_line(points: Vec<Vec3>, color: Color) {
    if points.len() < 2 {
        return;
    }
    if let Ok(mut line_buffer) = LINE_BUFFER.write() {
        line_buffer.push(LineData { points, color });
    }
}
