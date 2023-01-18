use crate::Gizmo;
use bevy::prelude::*;

#[derive(Debug, Default)]
pub(crate) struct GizmoInteractions {
    pub(crate) on_hover: Option<fn()>,
    pub(crate) on_click: Option<fn()>,
}

impl Gizmo {
    pub fn on_hover(mut self, func: fn()) -> Self {
        self.interactions.on_hover = Some(func);
        self
    }

    pub fn on_click(mut self, func: fn()) -> Self {
        self.interactions.on_click = Some(func);
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

pub(crate) fn interactions_handler(
    query: Query<(Option<&OnHover>, Option<&OnClick>, &Transform)>,
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
        // Cast a ray from the camera at the mouse position
        if let Some(ray) = camera.viewport_to_world(cam_transform, mouse_pos) {
            let origin = ray.origin - transform.translation;
            let closest_point = ray.direction.dot(origin);
            let distance = (origin - closest_point * ray.direction).length();

            // If the distance value if less than the gizmo
            // size then the gizmo is behind the cursor
            if distance < transform.scale.x {
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
    }
}
