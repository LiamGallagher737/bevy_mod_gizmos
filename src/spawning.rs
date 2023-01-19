use crate::{
    interactions::{OnClick, OnClickSystem, OnHover, OnHoverSystem},
    line::Line,
    Gizmo,
};
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub(crate) static ref GIZMO_SPAWN_BUFFER: RwLock<Vec<Gizmo>> = RwLock::new(vec![]);
    pub(crate) static ref LINE_SPAWN_BUFFER: RwLock<Vec<Line>> = RwLock::new(vec![]);
}

#[derive(Component)]
pub struct GizmoMarker;

pub(crate) fn spawn_gizmos(
    mut commands: Commands,
    mut mesh_handle: Local<Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if mesh_handle.is_weak() {
        let handle = meshes.add(Mesh::from(shape::Icosphere::default()));
        *mesh_handle = handle;
    }

    if let Ok(mut gizmos) = GIZMO_SPAWN_BUFFER.write() {
        while let Some(gizmo) = gizmos.pop() {
            let mut e = commands.spawn((
                PbrBundle {
                    transform: Transform::from_translation(gizmo.translation)
                        .with_scale(Vec3::splat(gizmo.size)),
                    mesh: mesh_handle.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: gizmo.color,
                        unlit: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                GizmoMarker,
                NotShadowCaster,
                NotShadowReceiver,
            ));

            if let Some(func) = gizmo.interactions.on_hover {
                e.insert(OnHover(func));
            }

            if let Some(func) = gizmo.interactions.on_click {
                e.insert(OnClick(func));
            }

            if let Some(func) = gizmo.interactions.on_hover_system {
                e.insert(OnHoverSystem(func));
            }

            if let Some(func) = gizmo.interactions.on_click_system {
                e.insert(OnClickSystem(func));
            }
        }
    }

    if let Ok(mut lines) = LINE_SPAWN_BUFFER.write() {
        while let Some(line) = lines.pop() {
            let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

            let mut vertices = vec![];
            for point in &line.points {
                vertices.push([point.x, point.y, point.z]);
            }
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

            mesh.insert_attribute(
                Mesh::ATTRIBUTE_NORMAL,
                vec![[0.0, 1.0, 0.0]; line.points.len()],
            );
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; line.points.len()]);

            let mut indices = vec![];
            for n in 0..line.points.len() as u16 {
                indices.push(n);
            }
            mesh.set_indices(Some(Indices::U16(indices)));

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(StandardMaterial {
                        base_color: line.color,
                        unlit: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                GizmoMarker,
                NotShadowCaster,
                NotShadowReceiver,
            ));
        }
    }
}

pub(crate) fn cleanup(mut commands: Commands, query: Query<Entity, With<GizmoMarker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
