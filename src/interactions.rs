use std::sync::RwLock;

use bevy::{
    ecs::event::Events,
    prelude::{Entity, World},
    utils::hashbrown::HashMap,
};
use bevy_mod_picking::{HoverEvent, PickingCameraBundle, PickingEvent};
use lazy_static::lazy_static;

use crate::Gizmo;

pub type GizmoInteractionCamera = PickingCameraBundle;

pub struct GizmoInteractionRaycastSet;

lazy_static! {
    pub(crate) static ref INTERACTIONS: RwLock<HashMap<Entity, GizmoInteractions>> =
        RwLock::new(HashMap::new());
}

#[derive(Clone)]
pub struct GizmoInteractions {
    pub(crate) lifetime: u8,
    pub on_click: Option<fn(&mut World)>,
    pub on_hover: Option<fn(&mut World)>,
}

impl GizmoInteractions {
    pub fn new() -> Self {
        Self {
            lifetime: 0,
            on_click: None,
            on_hover: None,
        }
    }
    pub fn has_some(&self) -> bool {
        if self.on_click.is_some() || self.on_hover.is_some() {
            true
        } else {
            false
        }
    }
    pub fn has_none(&self) -> bool {
        !self.has_some()
    }
}

impl Gizmo {
    pub fn on_click(mut self, on_click: fn(&mut World) -> ()) -> Self {
        self.interactions.on_click = Some(on_click);
        self
    }

    pub fn on_hover(mut self, on_click: fn(&mut World) -> ()) -> Self {
        self.interactions.on_hover = Some(on_click);
        self
    }
}

pub(crate) fn interaction_system(world: &mut World) {
    if let Ok(mut interactions) = INTERACTIONS.write() {
        let mut functions: Vec<fn(&mut World)> = vec![];
        if let Some(events) = world.get_resource::<Events<PickingEvent>>() {
            for event in events.get_reader().iter(&events) {
                match event {
                    PickingEvent::Clicked(entity) => {
                        if interactions.contains_key(entity) {
                            if let Some(f) = interactions.remove(entity).unwrap().on_click {
                                functions.push(f);
                            }
                        }
                    }
                    PickingEvent::Hover(hover) => {
                        if let HoverEvent::JustEntered(entity) = hover {
                            if interactions.contains_key(entity) {
                                if let Some(f) = interactions.remove(entity).unwrap().on_hover {
                                    functions.push(f);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        for func in functions {
            func(world);
        }
    }
}
