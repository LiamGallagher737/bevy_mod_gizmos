//! This contains the [`Gizmo`] type

use bevy::{
    math::{Quat, Vec3},
    prelude::{shape, Assets, Color, Handle, Mesh, ResMut, Transform},
};
use lazy_static::lazy_static;
use std::{
    fmt::{Display, Formatter, Result},
    sync::RwLock,
};

use crate::interactions::GizmoInteractions;

lazy_static! {
    static ref PRIMITIVE_MESH_HANDLES: RwLock<MeshHandles> = RwLock::new(MeshHandles::default());
}

#[derive(Default)]
struct MeshHandles {
    sphere: Handle<Mesh>,
    cube: Handle<Mesh>,
    capsule: Handle<Mesh>,
    torus: Handle<Mesh>,
}

pub(crate) fn setup(mut meshes: ResMut<Assets<Mesh>>) {
    if let Ok(mut handles) = PRIMITIVE_MESH_HANDLES.write() {
        handles.sphere = meshes.add(Mesh::from(shape::Icosphere::default()));
        handles.cube = meshes.add(Mesh::from(shape::Cube::default()));
        handles.capsule = meshes.add(Mesh::from(shape::Capsule::default()));
        handles.torus = meshes.add(Mesh::from(shape::Torus::default()));
    }
}

pub struct Gizmo {
    pub transform: Transform,
    pub color: Color,
    pub mesh_handle: Handle<Mesh>,
    pub interactions: GizmoInteractions,
}

impl Gizmo {
    /// Create a new gizmo with a custom mesh
    pub fn new(translation: Vec3, scale: Vec3, color: Color, mesh_handle: Handle<Mesh>) -> Self {
        Self {
            transform: Transform {
                translation,
                scale,
                ..Default::default()
            },
            color,
            mesh_handle,
            ..Default::default()
        }
    }

    /// Create a gizmo with a sphere mesh
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_mod_gizmos::*;
    ///
    /// let gizmo = Gizmo::sphere(Vec3::new(8.0, 2.0, 5.0), 0.4, Color::GREEN);
    /// ```
    pub fn sphere(translation: Vec3, diameter: f32, color: Color) -> Self {
        Self {
            transform: Transform {
                translation,
                scale: Vec3::new(diameter, diameter, diameter),
                ..Default::default()
            },
            color,
            mesh_handle: PRIMITIVE_MESH_HANDLES.read().unwrap().sphere.clone(),
            ..Default::default()
        }
    }

    /// Create a gizmo with a cube mesh
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_mod_gizmos::*;
    ///
    /// let gizmo = Gizmo::cube(Vec3::new(8.0, 2.0, 5.0), 0.4, Color::GREEN);
    /// ```
    pub fn cube(translation: Vec3, size: f32, color: Color) -> Self {
        Self {
            transform: Transform {
                translation,
                scale: Vec3::new(size, size, size),
                ..Default::default()
            },
            color,
            mesh_handle: PRIMITIVE_MESH_HANDLES.read().unwrap().cube.clone(),
            ..Default::default()
        }
    }

    /// Create a gizmo with a cubiod/box mesh
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_mod_gizmos::*;
    ///
    /// let gizmo = Gizmo::cubiod(Vec3::new(8.0, 2.0, 5.0), Vec3::new(3.0, 6.0, 9.0), Color::GREEN);
    /// ```
    pub fn cubiod(translation: Vec3, scale: Vec3, color: Color) -> Self {
        Self {
            transform: Transform {
                translation,
                scale,
                ..Default::default()
            },
            color,
            mesh_handle: PRIMITIVE_MESH_HANDLES.read().unwrap().cube.clone(),
            ..Default::default()
        }
    }

    /// Create a gizmo with a capsule mesh
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_mod_gizmos::*;
    ///
    /// let gizmo = Gizmo::capsule(Vec3::new(8.0, 2.0, 5.0), 1.5, 0.6, Color::GREEN);
    /// ```
    pub fn capsule(translation: Vec3, width: f32, height: f32, color: Color) -> Self {
        Self {
            transform: Transform {
                translation,
                scale: Vec3::new(width, height, width),
                ..Default::default()
            },
            color,
            mesh_handle: PRIMITIVE_MESH_HANDLES.read().unwrap().capsule.clone(),
            ..Default::default()
        }
    }

    /// Create a gizmo with a torus/donut mesh
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_mod_gizmos::*;
    ///
    /// let gizmo = Gizmo::torus(Vec3::new(8.0, 2.0, 5.0), 0.4, Color::GREEN);
    /// ```
    pub fn torus(translation: Vec3, size: f32, color: Color) -> Self {
        Self {
            transform: Transform {
                translation,
                scale: Vec3::new(size, size, size),
                ..Default::default()
            },
            color,
            mesh_handle: PRIMITIVE_MESH_HANDLES.read().unwrap().torus.clone(),
            ..Default::default()
        }
    }

    /// Change the gizmos positon
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_mod_gizmos::*;
    ///
    /// let gizmo = Gizmo::default().with_position(Vec3::new(8.0, 2.0, 5.0));
    /// ```
    pub fn with_position(mut self, translation: Vec3) -> Self {
        self.transform.translation = translation;
        self
    }

    /// Change the gizmos scale
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_mod_gizmos::*;
    ///
    /// let gizmo = Gizmo::default().with_scale(Vec3::new(3.0, 6.0, 9.0));
    /// ```
    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.transform.scale = scale;
        self
    }

    /// Change the gizmos rotation
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_mod_gizmos::*;
    ///
    /// let gizmo = Gizmo::default().with_rotation(Quat::from_xyzw(0.0, 0.7, 0.7, 0.0));
    /// ```
    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.transform.rotation = rotation;
        self
    }

    /// Change the gizmos color
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_mod_gizmos::*;
    ///
    /// let gizmo = Gizmo::default().with_color(Color::GREEN);
    /// ```
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Default for Gizmo {
    /// The default gizmo
    /// Positioned at the center of the world the default gizmo is exactly
    /// 1 unit wide, tall and deep. It's bright pink in color and perfectly round.
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            color: Color::default(),
            mesh_handle: PRIMITIVE_MESH_HANDLES.read().unwrap().sphere.clone(),
            interactions: GizmoInteractions::default(),
        }
    }
}

impl Display for Gizmo {
    /// Your average formatting
    /// # Example
    /// ``Gizmo::default()`` results in
    /// ```text
    /// Position: [0, 0, 0]
    /// Scale: [1, 1, 1],
    /// Rotation: [0, 0, 0, 1]
    /// Color: [1.0, 1.0, 1.0, 1.0]
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Position: {0} \nScale: {1}, \nRotation: {2} \nColor: {3:?}",
            self.transform.translation,
            self.transform.scale,
            self.transform.rotation,
            self.color.as_linear_rgba_f32()
        )
    }
}
