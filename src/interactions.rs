use std::sync::atomic::{AtomicUsize, Ordering};

use bevy::{prelude::{Component, EventReader, Commands, World}, math::Vec3};
use bevy_mod_picking::{PickingCameraBundle, PickingEvent};

use crate::Gizmo;

pub type GizmoInteractionCamera = PickingCameraBundle;

pub struct GizmoInteractionRaycastSet;

static INTERACTION_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Default, Component)]
pub struct GizmoInteractions {
    pub id: usize,
    pub on_click: Option<Box<dyn Fn(usize) + Send + Sync>>,
    pub on_hover: Option<Box<dyn Fn(usize) + Send + Sync>>,
}

impl GizmoInteractions {
    pub fn new() -> Self {
        let new_id = INTERACTION_ID_COUNTER.load(Ordering::Relaxed) + 1;
        INTERACTION_ID_COUNTER.store(new_id, Ordering::Relaxed);
        Self {
            id: new_id,
            on_click: None,
            on_hover: None,
        }
    }
}

impl Gizmo {
    pub fn on_click<F: Fn(usize) + Send + Sync + 'static>(&mut self, on_click: F) -> usize {
        self.interactions.on_click = Some(Box::new(on_click));
        self.interactions.id
    }

    pub fn on_hover<F: Fn(usize) + Send + Sync + 'static>(&mut self, on_click: F) -> usize {
        self.interactions.on_hover = Some(Box::new(on_click));
        self.interactions.id
    }
}

pub(crate) fn interaction_system(world: &World, mut events: EventReader<PickingEvent>) {
    INTERACTION_ID_COUNTER.store(0, Ordering::Relaxed);
    for event in events.iter() {
        match event {
            PickingEvent::Clicked(entity) => {
                if let Some(interactions) = world.entity(*entity).get::<GizmoInteractions>() {
                    if let Some(on_click) = &interactions.on_click {
                        // on_click(interactions.id);
                    }
                }
            },
            PickingEvent::Hover(event) => {
                let entity = match event {
                    bevy_mod_picking::HoverEvent::JustEntered(e) => e,
                    bevy_mod_picking::HoverEvent::JustLeft(e) => e,
                };
                if let Some(interactions) = world.entity(*entity).get::<GizmoInteractions>() {
                    if let Some(on_hover) = &interactions.on_hover {
                        // on_hover(interactions.id);
                    }
                }
            },
            _ => {},
        }
    }
}
