// Stop clippy complainging about the querys
#![allow(clippy::type_complexity)]

use crate::Gizmo;
use bevy::{ecs::schedule::SystemDescriptor, prelude::*};

#[derive(Debug, Default)]
pub(crate) struct GizmoInteractions {
    pub(crate) on_hover: Option<fn()>,
    pub(crate) on_click: Option<fn()>,
    pub(crate) on_hover_system: Option<SystemDescriptor>,
    pub(crate) on_click_system: Option<SystemDescriptor>,
}

impl Gizmo {
    /// Run the provided function when the gizmo is hovered
    pub fn on_hover(mut self, func: fn()) -> Self {
        self.interactions.on_hover = Some(func);
        self
    }

    /// Run the provided function when the gizmo is clicked on
    pub fn on_click(mut self, func: fn()) -> Self {
        self.interactions.on_click = Some(func);
        self
    }

    /// Run the provided function when the gizmo is hovered, you can use any bevy system parameters
    /// here but keep in mind no other systems can run at the same time as this so try to keep it short
    pub fn on_hover_system<Params>(mut self, func: impl IntoSystemDescriptor<Params>) -> Self {
        self.interactions.on_hover_system = Some(func.into_descriptor());
        self
    }

    /// Run the provided function when the gizmo is clicked on, you can use any bevy system parameters
    /// here but keep in mind no other systems can run at the same time as this so try to keep it short
    pub fn on_click_system<Params>(mut self, func: impl IntoSystemDescriptor<Params>) -> Self {
        self.interactions.on_click_system = Some(func.into_descriptor());
        self
    }
}

/// Add this to your main camera for interactable gizmos to function
#[derive(Component, Default)]
pub struct GizmoInteractionCamera;

#[derive(Component)]
pub struct OnHover(pub(crate) fn());

#[derive(Component)]
pub struct OnClick(pub(crate) fn());

#[derive(Component)]
pub struct OnHoverSystem(pub(crate) SystemDescriptor);

#[derive(Component)]
pub struct OnClickSystem(pub(crate) SystemDescriptor);

pub(crate) fn interactions_handler(
    query: Query<
        (Option<&OnHover>, Option<&OnClick>, &Transform),
        Or<(With<OnHover>, With<OnClick>)>,
    >,
    camera: Query<(&Camera, &GlobalTransform), With<GizmoInteractionCamera>>,
    mouse_btns: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    // Get the mouse position
    let mouse_pos = if let Some(Some(pos)) = windows.get_primary().map(|w| w.cursor_position()) {
        pos
    } else {
        // Most likely the cursor isn't within the window
        return;
    };

    // Get the gizmo interaction camera
    let (camera, cam_transform) = if let Ok(cam) = camera.get_single() {
        cam
    } else {
        return;
    };

    for (on_hover, on_click, transform) in query.iter() {
        let distance = gizmo_distance(camera, cam_transform, mouse_pos, transform.translation);
        if distance > transform.scale.x {
            continue;
        }
        // On Hover
        if let Some(on_hover) = on_hover {
            on_hover.0();
        }

        // On Click
        if let Some(on_click) = on_click {
            if !mouse_btns.just_pressed(MouseButton::Left) {
                continue;
            }
            on_click.0();
        }
    }
}

pub(crate) fn interactions_handler_system(world: &mut World) {
    let clicked = world
        .resource::<Input<MouseButton>>()
        .just_pressed(MouseButton::Left);

    // Get the mouse position
    let windows = world.resource::<Windows>();
    let mouse_pos = if let Some(Some(pos)) = windows.get_primary().map(|w| w.cursor_position()) {
        pos
    } else {
        // Most likely the cursor isn't within the window
        return;
    };

    // Get the gizmo camera
    let mut camera: QueryState<(&Camera, &GlobalTransform), With<GizmoInteractionCamera>> =
        world.query_filtered();
    let (camera, cam_transform) = if let Ok(cam) = camera.get_single(world) {
        (cam.0.to_owned(), cam.1.to_owned())
    } else {
        return;
    };

    let mut app = App::new();
    std::mem::swap(&mut app.world, world);

    let mut query: QueryState<
        Entity,
        (
            Or<(With<OnHoverSystem>, With<OnClickSystem>)>,
            With<Transform>,
        ),
    > = app.world.query_filtered();

    let entities: Vec<Entity> = query.iter(&app.world).collect();

    for e in entities {
        let on_hover = app.world.entity_mut(e).remove::<OnHoverSystem>();
        let on_click = app.world.entity_mut(e).remove::<OnClickSystem>();
        let transform = app.world.entity(e).get::<Transform>().unwrap();

        let distance = gizmo_distance(&camera, &cam_transform, mouse_pos, transform.translation);
        if distance > transform.scale.x {
            continue;
        }

        if let Some(on_hover) = on_hover {
            app.add_system(on_hover.0);
        }

        if clicked {
            if let Some(on_click) = on_click {
                app.add_system(on_click.0);
            }
        }
    }

    app.update();
    std::mem::swap(&mut app.world, world);
}

// Calculates the distance from the cursor to the gizmo
fn gizmo_distance(
    cam: &Camera,
    cam_transform: &GlobalTransform,
    mouse_pos: Vec2,
    gizmo_pos: Vec3,
) -> f32 {
    if let Some(ray) = cam.viewport_to_world(cam_transform, mouse_pos) {
        let origin = ray.origin - gizmo_pos;
        let closest_point = ray.direction.dot(origin);
        (origin - closest_point * ray.direction).length()
    } else {
        f32::INFINITY
    }
}
