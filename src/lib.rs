use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver, PbrBundle, StandardMaterial},
    prelude::{App, Assets, Color, Commands, Entity, Handle, Mesh, Plugin, ResMut, Transform},
    utils::hashbrown::HashMap,
};
use lazy_static::lazy_static;
use std::sync::RwLock;

pub mod gizmo_types;

pub struct GizmosPlugin;
impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GizmoEntities>();
        app.init_resource::<MaterialHandles>();
        app.add_startup_system(gizmo_types::setup);
        app.add_system(gizmos_system);
    }
}

pub trait Gizmo {
    fn get_transform(&self) -> Transform;
    fn get_color(&self) -> Color;
    fn get_mesh_handle(&self) -> Handle<Mesh>;
}

lazy_static! {
    static ref GIZMO_BUFFER: RwLock<Vec<GizmoData>> = RwLock::new(vec![]);
}

struct GizmoData {
    transform: Transform,
    color: Color,
    mesh_handle: Handle<Mesh>,
}

pub fn draw_gizmo<G: 'static + Gizmo>(gizmo: G) {
    if let Ok(mut gizmo_buffer) = GIZMO_BUFFER.write() {
        gizmo_buffer.push(GizmoData {
            transform: gizmo.get_transform(),
            color: gizmo.get_color(),
            mesh_handle: gizmo.get_mesh_handle(),
        });
    }
}

pub fn draw_gizmos<G: 'static + Gizmo>(mut gizmos: Vec<G>) {
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

fn gizmos_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_entities: ResMut<GizmoEntities>,
    mut material_handles: ResMut<MaterialHandles>,
) {
    while let Some(entity) = gizmo_entities.0.pop() {
        commands.entity(entity).despawn();
    }

    if let Ok(mut gizmos_buffer) = GIZMO_BUFFER.write() {
        while let Some(gizmo) = gizmos_buffer.pop() {
            let color_id = gizmo.color.as_linear_rgba_u32();
            let material = if material_handles.0.contains_key(&color_id) {
                material_handles.0[&color_id].clone()
            } else {
                let handle = materials.add(StandardMaterial {
                    base_color: gizmo.color,
                    unlit: true,
                    ..Default::default()
                });
                material_handles.0.insert(color_id, handle.clone());
                handle
            };

            gizmo_entities.0.push(
                commands
                    .spawn_bundle(PbrBundle {
                        transform: gizmo.transform,
                        mesh: gizmo.mesh_handle,
                        material,
                        ..Default::default()
                    })
                    .insert(NotShadowCaster)
                    .insert(NotShadowReceiver)
                    .id(),
            );
        }
    }
}

#[derive(Default)]
struct GizmoEntities(Vec<Entity>);
#[derive(Default)]
struct MaterialHandles(HashMap<u32, Handle<StandardMaterial>>);
