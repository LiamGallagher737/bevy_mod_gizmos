use bevy::{
    math::Vec3,
    prelude::{Color, Commands, Plugin},
};
use lazy_static::lazy_static;
use slotmap::{new_key_type, SlotMap};
use std::sync::RwLock;

mod tests;

new_key_type! {
    pub struct GizmoKey;
}

lazy_static! {
    static ref GIZMOS: RwLock<SlotMap<GizmoKey, Gizmo>> = RwLock::new(SlotMap::with_key());
}

pub struct GizmosContainer(SlotMap<GizmoKey, Gizmo>);

pub struct Gizmo {
    position: Vec3,
    color: Color,
}

pub struct Systems;
impl Plugin for Systems {
    fn build(&self, app: &mut bevy::prelude::App) {
        
    }
}

pub fn add_gizmo(position: Vec3, color: Color) -> Option<GizmoKey> {
    if let Ok(mut gizmos) = GIZMOS.write() {
        Some(gizmos.insert(Gizmo { position, color }))
    } else {
        None
    }
}

pub fn remove_gizmo(key: GizmoKey) -> Option<Gizmo> {
    if let Ok(mut gizmos) = GIZMOS.write() {
        gizmos.remove(key)
    } else {
        None
    }
}
