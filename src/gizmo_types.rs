use bevy::{
    math::Vec3,
    prelude::{Color, Handle, Mesh, Transform},
};

use crate::MESH_HANDLES;

pub trait Gizmo: Send + Sync {
    fn get_transform(&self) -> Transform;
    fn get_color(&self) -> Color;
    fn get_mesh_handle(&self) -> Handle<Mesh>;
}

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
        MESH_HANDLES.read().unwrap().sphere.clone()
    }
}

impl SphereGizmo {
    pub fn new(position: Vec3, diameter: f32, color: Color) -> Self {
        Self {
            position,
            diameter,
            color,
        }
    }
}

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
        MESH_HANDLES.read().unwrap().cube.clone()
    }
}

impl BoxGizmo {
    pub fn new(position: Vec3, scale: Vec3, color: Color) -> Self {
        Self {
            position,
            scale,
            color,
        }
    }
    pub fn new_cube(position: Vec3, size: f32, color: Color) -> Self {
        Self::new(position, Vec3::new(size, size, size), color)
    }
}

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
    pub fn new(position: Vec3, scale: Vec3, mesh_handle: Handle<Mesh>, color: Color) -> Self {
        Self {
            position,
            scale,
            mesh_handle,
            color,
        }
    }
}
