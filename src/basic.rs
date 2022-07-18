//! This contains the basic gizmo tools for drawing gizmos and lines

use bevy::math::Vec3;

use crate::*;

pub use crate::gizmo::Gizmo;

/// Draws a gizmo for a single frame
/// # Arguments
/// * `gizmo` - The gizmo to spawn, [How to create a gizmo](Gizmo)
/// # Example
/// ```
/// use bevy_mod_gizmos::*;
/// draw_gizmo(Gizmo::default());
/// ```
pub fn draw_gizmo(gizmo: Gizmo) {
    if let Ok(mut gizmo_buffer) = GIZMO_BUFFER.write() {
        gizmo_buffer.push(gizmo);
    }
}

/// Draws multiple gizmos for a single frame
/// # Arguments
/// * `gizmos` - The gizmos to spawn, this is a [`Vec`] of gizmos, [How to create a gizmo](Gizmo)
/// * `line` - Whether or not you want to draw a line between the gizmos
/// # Example
/// ```
/// use bevy_mod_gizmos::*;
/// draw_gizmos(vec![Gizmo::default(), Gizmo::default(), Gizmo::default()], true);
/// ```
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
/// * `points` - A [`Vec`] of [`Vec3`] holding the line points
/// * `color` - The color of the line
/// # Example
/// ```
/// use bevy_mod_gizmos::*;
/// draw_line(vec![Vec3::new(8.0, 2.0, 5.0), Vec3::new(9.0, 3.0, 6.0), Vec3::new(10.0, 4.0, 7.0)], Color::GREEN);
/// ```
pub fn draw_line(points: Vec<Vec3>, color: Color) {
    if points.len() < 2 {
        return;
    }
    if let Ok(mut line_buffer) = LINE_BUFFER.write() {
        line_buffer.push(LineData { points, color });
    }
}
