use bevy::{
    math::Vec3,
    prelude::{Color, Handle, Mesh, Transform},
};

use crate::MESH_HANDLES;

pub trait Gizmo {
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

pub struct CubeGizmo {
    pub position: Vec3,
    pub size: f32,
    pub color: Color,
}

impl Gizmo for CubeGizmo {
    fn get_transform(&self) -> Transform {
        Transform {
            translation: self.position,
            scale: Vec3::new(self.size, self.size, self.size),
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

pub struct BoxGizmo {
    pub position: Vec3,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub color: Color,
}

impl Gizmo for BoxGizmo {
    fn get_transform(&self) -> Transform {
        Transform {
            translation: self.position,
            scale: Vec3::new(self.width, self.height, self.depth),
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
