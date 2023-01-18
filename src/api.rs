use crate::{
    spawning::{GIZMO_SPAWN_BUFFER, LINE_SPAWN_BUFFER},
    Gizmo, Line,
};
use bevy::prelude::*;

/// Draw a single gizmo, it takes anything that implements `Into<Gizmo>`
pub fn draw_gizmo<G: Into<Gizmo>>(gizmo: G) {
    if let Ok(mut gizmos) = GIZMO_SPAWN_BUFFER.write() {
        gizmos.push(gizmo.into());
    }
}

/// Draw multiple gizmos, it takes a vec of anything that implements `Into<Gizmo>`
pub fn draw_gizmos<G: Into<Gizmo>>(gizmos: Vec<G>) {
    if let Ok(mut gizmos_buf) = GIZMO_SPAWN_BUFFER.write() {
        for gizmo in gizmos {
            gizmos_buf.push(gizmo.into());
        }
    }
}

/// Draw multiple gizmos with a connecting line, it takes a vec of anything that implements `Into<Gizmo>`
pub fn draw_gizmos_with_line<G: Into<Gizmo>>(mut gizmos: Vec<G>) {
    let gizmos: Vec<Gizmo> = gizmos.drain(..).map(|g| g.into()).collect();
    draw_line(
        gizmos.iter().map(|g| g.translation).collect(),
        gizmos[0].color,
    );
    draw_gizmos(gizmos);
}

/// Draw a line, it takes a vec of Vec3 points and a color
pub fn draw_line(points: Vec<Vec3>, color: Color) {
    if let Ok(mut lines) = LINE_SPAWN_BUFFER.write() {
        lines.push(Line::new(points, color));
    }
}

/// Draw a closed line, it takes a vec of Vec3 points and a color
pub fn draw_closed_line(mut points: Vec<Vec3>, color: Color) {
    if points.is_empty() {
        return;
    }
    points.push(points[0]);
    if let Ok(mut lines) = LINE_SPAWN_BUFFER.write() {
        lines.push(Line::new(points, color));
    }
}
