use std::sync::RwLock;

use bevy::{
    ecs::event::Events,
    prelude::{Entity, World},
    utils::hashbrown::HashMap,
};
use bevy_mod_picking::{PickingCameraBundle, PickingEvent};
use lazy_static::lazy_static;

use crate::Gizmo;

pub type GizmoInteractionCamera = PickingCameraBundle;

pub struct GizmoInteractionRaycastSet;

lazy_static! {
    pub(crate) static ref INTERACTIONS: RwLock<HashMap<Entity, GizmoInteractions>> =
        RwLock::new(HashMap::new());
}

#[derive(Clone, Default)]
pub struct GizmoInteractions {
    pub(crate) lifetime: u8,
    // May add more interactions in the future like hover
    pub on_click: Option<fn(&mut World)>,
}

impl GizmoInteractions {
    pub fn has_some(&self) -> bool {
        self.on_click.is_some()
    }
    pub fn has_none(&self) -> bool {
        !self.has_some()
    }
}

impl Gizmo {
    /// Change the gizmos on_click interaction
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_mod_gizmos::*;
    ///
    /// let gizmo = Gizmo::default().on_click(|world| {
    ///
    ///     // What to do when clicked, you have full access to the ECS world though
    ///     // be aware that while this runs no other systems can run in parallel
    ///
    ///     let mut query = world.query::<&mut Transform>();
    ///     for mut transform in query.iter_mut(world) {
    ///         transform.translation.x += 0.1;
    ///     }
    /// });
    /// ```
    pub fn on_click(mut self, on_click: fn(&mut World) -> ()) -> Self {
        self.interactions.on_click = Some(on_click);
        self
    }
}

pub(crate) fn interaction_system(world: &mut World) {
    if let Ok(mut interactions) = INTERACTIONS.write() {
        let mut functions: Vec<fn(&mut World)> = vec![];
        if let Some(events) = world.get_resource::<Events<PickingEvent>>() {
            for event in events.get_reader().iter(events) {
                if let PickingEvent::Clicked(entity) = event {
                    if interactions.contains_key(entity) {
                        if let Some(f) = interactions.remove(entity).unwrap().on_click {
                            functions.push(f);
                        }
                    }
                }
            }
        }
        for func in functions {
            func(world);
        }
    }
}
