use bevy::{
    pbr::{PbrBundle, StandardMaterial},
    prelude::{
        shape, App, Assets, Commands, Entity, Handle, Mesh, Plugin, ResMut, CoreStage,
    },
};
use gizmo_types::*;
use lazy_static::lazy_static;
use slotmap::{new_key_type, SlotMap};
use std::sync::RwLock;

pub mod gizmo_types;
mod tests;

new_key_type! {
    pub struct GizmoKey;
}

lazy_static! {
    static ref GIZMOS: RwLock<SlotMap<GizmoKey, (Option<Entity>, Box<dyn Gizmo + Send + Sync>)>> = RwLock::new(SlotMap::with_key());
    static ref GIZMO_SPAWN_BUFFER: RwLock<Vec<GizmoKey>> = RwLock::new(Vec::new());
    static ref GIZMO_DESPAWN_BUFFER: RwLock<Vec<GizmoKey>> = RwLock::new(Vec::new());
    static ref GIZMO_TEMP_BUFFER: RwLock<Vec<GizmoKey>> = RwLock::new(Vec::new());
    static ref MESH_HANDLES: RwLock<MeshHandles> = RwLock::new(MeshHandles::default());
}

#[derive(Default)]
struct MeshHandles {
    sphere: Handle<Mesh>,
    cube: Handle<Mesh>,
}

pub struct GizmosPlugin;
impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system(spawn_gizmos);
        app.add_system(despawn_gizmos);
        app.add_system_to_stage(CoreStage::First, despawn_temp_gizmos);
    }
}

fn setup(mut meshes: ResMut<Assets<Mesh>>, _materials: ResMut<Assets<StandardMaterial>>) {
    if let Ok(mut handles) = MESH_HANDLES.write() {
        handles.sphere = meshes.add(Mesh::from(shape::Icosphere::default()));
        handles.cube = meshes.add(Mesh::from(shape::Box::default()));
    }
}

fn spawn_gizmos(mut commands: Commands) {
    if let Ok(mut buffer) = GIZMO_SPAWN_BUFFER.write() {
        if buffer.is_empty() {
            return;
        }

        if let Ok(mut gizmos) = GIZMOS.write() {
            while let Some(key) = buffer.pop() {
                if let Some(value) = gizmos.get_mut(key) {
                    let entity = commands
                        .spawn_bundle(PbrBundle {
                            transform: value.1.get_transform(),
                            mesh: value.1.get_mesh_handle(),
                            ..Default::default()
                        })
                        .id();
                    value.0 = Some(entity);
                }
            }
        }
    }
}

fn despawn_gizmos(mut commands: Commands) {
    if let Ok(mut buffer) = GIZMO_DESPAWN_BUFFER.write() {
        if buffer.is_empty() {
            return;
        }

        if let Ok(mut gizmos) = GIZMOS.write() {
            while let Some(key) = buffer.pop() {
                if let Some((e, _)) = gizmos.get(key) {
                    if let Some(entity) = e {
                        commands.entity(*entity).despawn();
                    }
                    gizmos.remove(key);
                }
            }
        }
    }
}

fn despawn_temp_gizmos(mut commands: Commands) {
    if let Ok(mut buffer) = GIZMO_TEMP_BUFFER.write() {
        if buffer.is_empty() {
            return;
        }

        if let Ok(mut gizmos) = GIZMOS.write() {
            while let Some(key) = buffer.pop() {
                if let Some((e, _)) = gizmos.get(key) {
                    if let Some(entity) = e {
                        commands.entity(*entity).despawn();
                    }
                    gizmos.remove(key);
                }
            }
        }
    }
}

pub fn add_gizmo<G: 'static + Gizmo + Send + Sync>(gizmo: G) -> Option<GizmoKey> {
    if let Ok(mut gizmos) = GIZMOS.write() {
        if let Ok(mut buffer) = GIZMO_SPAWN_BUFFER.write() {
            let key = gizmos.insert((None, Box::new(gizmo)));
            buffer.push(key);
            Some(key)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn draw_gizmo<G: 'static + Gizmo + Send + Sync>(gizmo: G) -> Option<GizmoKey> {
    if let Ok(mut gizmos) = GIZMOS.write() {
        if let Ok(mut buffer) = GIZMO_SPAWN_BUFFER.write() {
            if let Ok(mut temp_gizmos) = GIZMO_TEMP_BUFFER.write() {
                let key = gizmos.insert((None, Box::new(gizmo)));
                buffer.push(key);
                temp_gizmos.push(key);
                Some(key)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

pub fn remove_gizmo(key: GizmoKey) {
    if let Ok(mut gizmos) = GIZMOS.write() {
        if !gizmos.contains_key(key) {
            return;
        }
        if let Ok(mut buffer) = GIZMO_DESPAWN_BUFFER.write() {
            buffer.push(key);
            gizmos.remove(key);
        }
    }
}
