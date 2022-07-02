use bevy::{
    pbr::{PbrBundle, StandardMaterial, NotShadowCaster, NotShadowReceiver},
    prelude::{shape, App, Assets, Commands, CoreStage, Entity, Handle, Mesh, Plugin, ResMut}, utils::hashbrown::HashMap,
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

impl GizmoKey {
    pub fn remove(self) {
        remove_gizmo(self);
    }
}

lazy_static! {
    static ref GIZMOS: RwLock<SlotMap<GizmoKey, (Option<Entity>, Box<dyn Gizmo>)>> =
        RwLock::new(SlotMap::with_key());
    static ref GIZMO_SPAWN_BUFFER: RwLock<Vec<GizmoKey>> = RwLock::new(Vec::new());
    static ref GIZMO_DESPAWN_BUFFER: RwLock<Vec<GizmoKey>> = RwLock::new(Vec::new());
    static ref GIZMO_TEMP_BUFFER: RwLock<Vec<GizmoKey>> = RwLock::new(Vec::new());
    static ref MESH_HANDLES: RwLock<MeshHandles> = RwLock::new(MeshHandles::default());
    static ref MATERIAL_HANDLES: RwLock<HashMap<u32, Handle<StandardMaterial>>> = RwLock::new(HashMap::new());
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

fn setup(mut meshes: ResMut<Assets<Mesh>>) {
    if let Ok(mut handles) = MESH_HANDLES.write() {
        handles.sphere = meshes.add(Mesh::from(shape::Icosphere::default()));
        handles.cube = meshes.add(Mesh::from(shape::Cube::default()));
    }
}

fn spawn_gizmos(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    if let (Ok(mut gizmos), Ok(mut buffer)) = (GIZMOS.write(), GIZMO_SPAWN_BUFFER.write()) {
        if buffer.is_empty() {
            return;
        }

        while let Some(key) = buffer.pop() {
            if let (Some(value), Ok(mut material_handles)) = (gizmos.get_mut(key), MATERIAL_HANDLES.write()) {
                let material = {
                    if let Some(handle) = material_handles.get(&value.1.get_color().as_linear_rgba_u32()) {
                        handle.to_owned()
                    } else {
                        let m = materials.add(StandardMaterial {
                            base_color: value.1.get_color(),
                            unlit: true,
                            ..Default::default()
                        });
                        material_handles.insert(value.1.get_color().as_linear_rgba_u32(), m.clone());
                        m
                    }
                };
                let entity = commands
                    .spawn_bundle(PbrBundle {
                        transform: value.1.get_transform(),
                        mesh: value.1.get_mesh_handle(),
                        material,
                        ..Default::default()
                    })
                    .insert(NotShadowCaster)
                    .insert(NotShadowReceiver)
                    .id();
                value.0 = Some(entity);
            }
        }
    }
}

fn despawn_gizmos(mut commands: Commands) {
    if let (Ok(mut gizmos), Ok(mut buffer)) = (GIZMOS.write(), GIZMO_DESPAWN_BUFFER.write()) {
        if buffer.is_empty() {
            return;
        }

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

fn despawn_temp_gizmos(mut commands: Commands) {
    if let (Ok(mut gizmos), Ok(mut buffer)) = (GIZMOS.write(), GIZMO_TEMP_BUFFER.write()) {
        if buffer.is_empty() {
            return;
        }

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

pub fn add_gizmo<G: 'static + Gizmo>(gizmo: G) -> Option<GizmoKey> {
    if let (Ok(mut gizmos), Ok(mut buffer)) = (GIZMOS.write(), GIZMO_SPAWN_BUFFER.write()) {
        let key = gizmos.insert((None, Box::new(gizmo)));
        buffer.push(key);
        Some(key)
    } else {
        None
    }
}

pub fn draw_gizmo<G: 'static + Gizmo>(gizmo: G) -> Option<GizmoKey> {
    if let (Ok(mut temp_gizmos), Some(key)) = (GIZMO_TEMP_BUFFER.write(), add_gizmo(gizmo)) {
        temp_gizmos.push(key);
        Some(key)
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

pub fn add_gizmos<G: 'static + Gizmo>(gizmos: Vec<G>) -> Vec<GizmoKey> {
    let mut gizmo_keys = vec![];
    for gizmo in gizmos {
        if let Some(key) = add_gizmo(gizmo) {
            gizmo_keys.push(key)
        }
    }
    gizmo_keys
}

pub fn draw_gizmos<G: 'static + Gizmo>(gizmos: Vec<G>) -> Vec<GizmoKey> {
    let mut gizmo_keys = vec![];
    for gizmo in gizmos {
        if let Some(key) = draw_gizmo(gizmo) {
            gizmo_keys.push(key)
        }
    }
    gizmo_keys
}

pub fn remmove_gizmos(keys: Vec<GizmoKey>) {
    for key in keys {
        remove_gizmo(key);
    }
}
