use bevy::{
    math::Vec3,
    prelude::{shape, Assets, Color, Handle, Mesh, ResMut, Transform},
};
use lazy_static::lazy_static;
use std::sync::RwLock;

use crate::Gizmo;

lazy_static! {
    static ref PRIMITIVE_MESH_HANDLES: RwLock<MeshHandles> = RwLock::new(MeshHandles::default());
}

#[derive(Default)]
struct MeshHandles {
    sphere: Handle<Mesh>,
    cube: Handle<Mesh>,
}

pub(crate) fn setup(mut meshes: ResMut<Assets<Mesh>>) {
    if let Ok(mut handles) = PRIMITIVE_MESH_HANDLES.write() {
        handles.sphere = meshes.add(Mesh::from(shape::Icosphere::default()));
        handles.cube = meshes.add(Mesh::from(shape::Cube::default()));
    }
}

/// A spherial [`Gizmo`]
/// # Fields
/// * `position` - The position in world space of the gizmo
/// * `diameter` - The size of the gizmo
/// * `color` - The color of the gizmo
pub struct SphereGizmo {
    pub position: Vec3,
    pub diameter: f32,
    pub color: Color,
}

impl Gizmo for SphereGizmo {
    fn get_transform(&self) -> Transform {
        Transform {
            translation: self.position,
            scale: Vec3::new(self.diameter, self.diameter, self.diameter),
            ..Default::default()
        }
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_mesh_handle(&self) -> Handle<Mesh> {
        PRIMITIVE_MESH_HANDLES.read().unwrap().sphere.clone()
    }
}

impl SphereGizmo {
    /// Returns a new [`SphereGizmo`] with it's position, diameter and color
    pub fn new(position: Vec3, diameter: f32, color: Color) -> Self {
        Self {
            position,
            diameter,
            color,
        }
    }
}

/// A box [`Gizmo`]
/// # Fields
/// * `position` - The position in world space of the gizmo
/// * `scale` - The size of the gizmo
/// * `color` - The color of the gizmo
pub struct BoxGizmo {
    pub position: Vec3,
    pub scale: Vec3,
    pub color: Color,
}

impl Gizmo for BoxGizmo {
    fn get_transform(&self) -> Transform {
        Transform {
            translation: self.position,
            scale: self.scale,
            ..Default::default()
        }
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_mesh_handle(&self) -> Handle<Mesh> {
        PRIMITIVE_MESH_HANDLES.read().unwrap().cube.clone()
    }
}

impl BoxGizmo {
    /// Returns a new [`BoxGizmo`] with it's position, scale and color
    pub fn new(position: Vec3, scale: Vec3, color: Color) -> Self {
        Self {
            position,
            scale,
            color,
        }
    }
    /// Returns a new [`BoxGizmo`] with it's position, scale based on size input and color
    pub fn new_cube(position: Vec3, size: f32, color: Color) -> Self {
        Self::new(position, Vec3::new(size, size, size), color)
    }
}

/// A spherial [`Gizmo`]
/// # Fields
/// * `position` - The position in world space of the gizmo
/// * `scale` - The size of the gizmo
/// * `mesh_handle` - A [`Handle`] of the [`Mesh`] for the gizmo
/// * `color` - The color of the gizmo
pub struct MeshGizmo {
    pub position: Vec3,
    pub scale: Vec3,
    pub mesh_handle: Handle<Mesh>,
    pub color: Color,
}

impl Gizmo for MeshGizmo {
    fn get_transform(&self) -> Transform {
        Transform {
            translation: self.position,
            scale: self.scale,
            ..Default::default()
        }
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_mesh_handle(&self) -> Handle<Mesh> {
        self.mesh_handle.clone()
    }
}

impl MeshGizmo {
    /// Returns a new [`MeshGizmo`] with it's position, scale, mesh_handle and color
    pub fn new(position: Vec3, scale: Vec3, mesh_handle: Handle<Mesh>, color: Color) -> Self {
        Self {
            position,
            scale,
            mesh_handle,
            color,
        }
    }
}
