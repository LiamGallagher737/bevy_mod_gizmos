//! # Examples
//!
//! Draw a single gizmo
//! ```
//! # use bevy::prelude::*;
//! # use bevy_mod_gizmos::*;
//! draw_gizmo(
//!     Gizmo::new(Vec3::ZERO, 1.0, Color::WHITE)
//!         .on_click(|| println!("Clicked!"))
//!         .on_hover(|| println!("Hovered")),
//! );
//! ```
//!
//! Draw multiple gizmos
//! ```
//! # use bevy::prelude::*;
//! # use bevy_mod_gizmos::*;
//! draw_gizmos(vec![
//!     Gizmo::new(Vec3::X, 0.5, Color::RED),
//!     Gizmo::new(Vec3::Y, 0.5, Color::RED),
//!     Gizmo::new(Vec3::Z, 0.5, Color::RED),
//! ]);
//! ```
//!
//! Draw a line
//! ```
//! # use bevy::prelude::*;
//! # use bevy_mod_gizmos::*;
//! draw_line(
//!     vec![
//!         Vec3::X,
//!         Vec3::Y,
//!         Vec3::Z,
//!     ],
//!     Color::PURPLE,
//! );
//! ```
//!
//! The rest of the methods can be found <a href="#functions">here</a>.

use bevy::prelude::*;
use interactions::interactions_handler;
use spawning::{cleanup, spawn_gizmos};

mod api;
mod gizmo;
mod interactions;
mod line;
mod spawning;

pub use api::*;
pub use gizmo::*;
pub use interactions::GizmoInteractionCamera;
pub use line::*;

/// Add this to your bevy [`App`] for gizmos to function
pub struct GizmosPlugin;
impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        // Using First instead of PreUpdate causes issues with bevy_editor_pls
        app.add_system_to_stage(CoreStage::PreUpdate, cleanup);
        app.add_system_to_stage(CoreStage::PreUpdate, spawn_gizmos.after(cleanup));
        app.add_system_to_stage(CoreStage::PostUpdate, interactions_handler);
    }
}
