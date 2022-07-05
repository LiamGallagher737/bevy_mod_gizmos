use std::sync::atomic::{AtomicUsize, Ordering};

use bevy::{prelude::{Component, World}, ecs::event::Events};
use bevy_mod_picking::{PickingCameraBundle, PickingEvent};

use crate::Gizmo;

pub type GizmoInteractionCamera = PickingCameraBundle;

pub struct GizmoInteractionRaycastSet;

static INTERACTION_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Component)]
pub struct GizmoInteractions {
    pub id: usize,
    pub on_click: Option<fn(&mut World)>,
    pub on_hover: Option<fn(&mut World)>,
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
    INTERACTION_ID_COUNTER.store(0, Ordering::Relaxed);
    let mut functions = vec![];
    if let Some(events) = world.get_resource::<Events<PickingEvent>>() {
        for event in events.get_reader().iter(&events) {
            match event {
                PickingEvent::Clicked(entity) => {
                    if let Some(interactions) = world.entity(*entity).get::<GizmoInteractions>() {
                        if let Some(on_click) = &interactions.on_click {
                            functions.push(on_click.clone());
                        }
                    }
                },
                PickingEvent::Hover(_hover) => {
                    // println!("{:#?}", hover);
                },
                _ => {}
            }
        }
    }

    for func in functions {
        func(world);
    }
}
