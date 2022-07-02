use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver, PbrBundle, StandardMaterial},
    prelude::{App, Assets, Color, Commands, Entity, Handle, Mesh, Plugin, ResMut, Transform, CoreStage},
    utils::hashbrown::HashMap, math::Vec3, render::mesh::{PrimitiveTopology, Indices},
};
use lazy_static::lazy_static;
use std::sync::RwLock;

pub mod basic;
pub mod gizmo_types;

pub use basic::*;

/// Add this to your bevy [`App`] to function
pub struct GizmosPlugin;
impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GizmoEntities>();
        app.init_resource::<MaterialHandles>();
        app.add_startup_system(gizmo_types::setup);
        app.add_system_to_stage(CoreStage::First, cleanup_system);
        app.add_system(gizmos_system);
        app.add_system(lines_system);
    }
}

/// Implement this for a struct to use it as a gizmo
pub trait Gizmo {
    /// Construct a transform for the gizmo entity to use
    fn get_transform(&self) -> Transform;
    /// Return the color for the gizmo to use
    fn get_color(&self) -> Color;
    /// Return a handle of the mesh for the gizmo to use
    fn get_mesh_handle(&self) -> Handle<Mesh>;
}

lazy_static! {
    /// Gizmos to spawn next time the system runs
    static ref GIZMO_BUFFER: RwLock<Vec<GizmoData>> = RwLock::new(vec![]);
    /// Lines to spawn next time the system runs
    static ref LINE_BUFFER: RwLock<Vec<LineData>> = RwLock::new(vec![]);

    // Mesh handles to remove next frame
    static ref TEMP_MESH_HANDLES: RwLock<Vec<Handle<Mesh>>> = RwLock::new(vec![]);
}

struct GizmoData {
    /// Transform of the gizmo
    transform: Transform,
    /// Color of the gizmo
    color: Color,
    /// Handle for the mesh the gizmo will use
    mesh_handle: Handle<Mesh>,
}

struct LineData {
    points: Vec<Vec3>,
    color: Color,
}

/// This is where the [`GizmoData`] objects in [`static@GIZMO_BUFFER`] is use to create entities
fn gizmos_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_entities: ResMut<GizmoEntities>,
    mut material_handles: ResMut<MaterialHandles>,
) {
    if let Ok(mut gizmo_buffer) = GIZMO_BUFFER.write() {
        while let Some(gizmo) = gizmo_buffer.pop() {
            let material_handle = get_material_handle(gizmo.color, &mut material_handles, &mut materials);
            gizmo_entities.0.push(
                commands
                    .spawn_bundle(PbrBundle {
                        transform: gizmo.transform,
                        mesh: gizmo.mesh_handle,
                        material: material_handle,
                        ..Default::default()
                    })
                    .insert(NotShadowCaster)
                    .insert(NotShadowReceiver)
                    .id(),
            );
        }
    }
}

/// This is where the [`LineData`] objects in [`static@LINE_BUFFER`] is use to create entities
fn lines_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_entities: ResMut<GizmoEntities>,
    mut material_handles: ResMut<MaterialHandles>,
) {
    if let (Ok(mut line_buffer), Ok(mut temp_mesh_handles)) = (LINE_BUFFER.write(), TEMP_MESH_HANDLES.write()) {
        while let Some(line) = line_buffer.pop() {

            let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

            let mut vertices = vec![];
            for point in &line.points {
                vertices.push([point.x, point.y, point.z]);
            }
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; line.points.len()]);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; line.points.len()]);

            let mut indices = vec![];
            for n in 0..line.points.len() as u16 {
                indices.push(n);
            }
            mesh.set_indices(Some(Indices::U16(indices)));

            let mesh_handle = meshes.add(mesh);
            temp_mesh_handles.push(mesh_handle.clone());

            let material_handle = get_material_handle(line.color, &mut material_handles, &mut materials);
            gizmo_entities.0.push(
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: mesh_handle,
                        material: material_handle,
                        ..Default::default()
                    })
                    .id()
            );
        }
    }
}

/// This system despawns all entities from the last frame
/// and removes temp mesh handles,
/// it runs in [`CoreStage::First`]
fn cleanup_system(
    mut commands: Commands,
    mut gizmo_entities: ResMut<GizmoEntities>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    while let Some(entity) = gizmo_entities.0.pop() {
        commands.entity(entity).despawn();
    }
    if let Ok(mut temp_mesh_handles) = TEMP_MESH_HANDLES.write() {
        while let Some(mesh_handle) = temp_mesh_handles.pop() {
            meshes.remove(mesh_handle);
        }
    }
}

/// If we have a [`StandardMaterial`] already with the same [`Color`] then we return 
/// the [`Handle`] to that,  else we create a new material and return its [`Handle`]
fn get_material_handle(color: Color, material_handles: &mut ResMut<MaterialHandles>, materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
    let color_id = color.as_linear_rgba_u32();
    let material = if material_handles.0.contains_key(&color_id) {
        material_handles.0[&color_id].clone()
    } else {
        let handle = materials.add(StandardMaterial {
            base_color: color,
            unlit: true,
            ..Default::default()
        });
        material_handles.0.insert(color_id, handle.clone());
        handle
    };
    material
}

#[derive(Default)]
struct GizmoEntities(Vec<Entity>);
#[derive(Default)]
struct MaterialHandles(HashMap<u32, Handle<StandardMaterial>>);
