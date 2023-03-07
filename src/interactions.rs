// Stop clippy complainging about the querys
#![allow(clippy::type_complexity)]

use crate::Gizmo;
use bevy::{ecs::schedule::SystemConfig, prelude::*, window::PrimaryWindow};

#[derive(Default)]
pub(crate) struct GizmoInteractions {
    pub(crate) on_hover: Option<fn()>,
    pub(crate) on_click: Option<fn()>,
    pub(crate) on_hover_system: Option<SystemConfig>,
    pub(crate) on_click_system: Option<SystemConfig>,
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
    pub fn on_hover_system<Params>(mut self, func: impl IntoSystemConfig<Params>) -> Self {
        self.interactions.on_hover_system = Some(func.into_config());
        self
    }

    /// Run the provided function when the gizmo is clicked on, you can use any bevy system parameters
    /// here but keep in mind no other systems can run at the same time as this so try to keep it short
    pub fn on_click_system<Params>(mut self, func: impl IntoSystemConfig<Params>) -> Self {
        self.interactions.on_click_system = Some(func.into_config());
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
pub struct OnHoverSystem(pub(crate) SystemConfig);

#[derive(Component)]
pub struct OnClickSystem(pub(crate) SystemConfig);

pub(crate) fn interactions_handler(
    query: Query<
        (Option<&OnHover>, Option<&OnClick>, &Transform),
        Or<(With<OnHover>, With<OnClick>)>,
    >,
    camera: Query<(&Camera, &GlobalTransform), With<GizmoInteractionCamera>>,
    mouse_btns: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    // Get the mouse position
    let mouse_pos =
        if let Ok(Some(pos)) = window.get_single().map(|window| window.cursor_position()) {
            pos
        } else {
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

pub(crate) fn system_interactions_handler(world: &mut World) {
    let clicked = world
        .resource::<Input<MouseButton>>()
        .just_pressed(MouseButton::Left);

    // Get the mouse position
    let mut window = world.query_filtered::<&Window, With<PrimaryWindow>>();
    let mouse_pos = if let Ok(Some(pos)) = window
        .get_single(world)
        .map(|window| window.cursor_position())
    {
        pos
    } else {
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

    let mut schedule = Schedule::default();

    // Query all gizmo entities with eiter a hover or click interaction or both
    let mut query: QueryState<
        Entity,
        (
            Or<(With<OnHoverSystem>, With<OnClickSystem>)>,
            With<Transform>,
        ),
    > = world.query_filtered();

    // Get the query entities
    let entities: Vec<Entity> = query.iter(world).collect();

    for e in entities {
        let on_hover = world.entity_mut(e).take::<OnHoverSystem>();
        let on_click = world.entity_mut(e).take::<OnClickSystem>();
        let transform = world.entity(e).get::<Transform>().unwrap();

        // Check if hovered by cursor
        let distance = gizmo_distance(&camera, &cam_transform, mouse_pos, transform.translation);
        if distance > transform.scale.x {
            continue;
        }

        // Hover interaction
        if let Some(on_hover) = on_hover {
            schedule.add_system(on_hover.0);
        }

        // Clicked interaction
        if clicked {
            if let Some(on_click) = on_click {
                schedule.add_system(on_click.0);
            }
        }
    }

    // Run the schedule on the world
    schedule.run(world);
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
