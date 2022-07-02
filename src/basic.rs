//! This contains just the basic things for gizmos

use bevy::math::Vec3;

use crate::*;

pub use crate::gizmo_types::{BoxGizmo, MeshGizmo, SphereGizmo};

/// Draw a gizmo for a signle frame
/// # Arguments
/// * `gizmo` - The gizmo to spawn, this can be any struct that implements [`Gizmo`]
pub fn draw_gizmo<G: 'static + Gizmo>(gizmo: G) {
    if let Ok(mut gizmo_buffer) = GIZMO_BUFFER.write() {
        gizmo_buffer.push(GizmoData {
            transform: gizmo.get_transform(),
            color: gizmo.get_color(),
            mesh_handle: gizmo.get_mesh_handle(),
        });
    }
}

/// Draw multiple gizmo for a signle frame
/// # Arguments
/// * `gizmos` - The gizmos to spawn, this is a [`Vec`] of any struct that implements [`Gizmo`]
pub fn draw_gizmos<G: 'static + Gizmo>(mut gizmos: Vec<G>, line: bool) {
    if gizmos.is_empty() {
        return;
    }
    if line {
        draw_line(
            gizmos.iter().map(|g| g.get_transform().translation).collect(),
            gizmos[0].get_color(),
        );
    }
    if let Ok(mut gizmo_buffer) = GIZMO_BUFFER.write() {
        while let Some(gizmo) = gizmos.pop() {
            gizmo_buffer.push(GizmoData {
                transform: gizmo.get_transform(),
                color: gizmo.get_color(),
                mesh_handle: gizmo.get_mesh_handle(),
            });
        }
    }
}

pub fn draw_line(points: Vec<Vec3>, color: Color) {
    if let Ok(mut line_buffer) = LINE_BUFFER.write() {
        line_buffer.push(LineData {
            points,
            color,
        });
    }
}
