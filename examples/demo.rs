use bevy::prelude::*;
use bevy_mod_gizmos::*;

#[rustfmt::skip]
fn main() {
    App::new()

        .add_plugins(DefaultPlugins)
        .add_plugin(GizmosPlugin)

        .add_startup_system(setup)
        .add_system(update)

        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        
        .run();
}

fn setup(mut commands: Commands) {
    println!("Trying hoering and clicking the center gizmo");

    let cam_transform = Transform::from_xyz(0.0, 0.0, 8.0);
    commands.spawn((
        Camera3dBundle {
            transform: cam_transform.looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        GizmoInteractionCamera,
    ));
}

fn update() {
    draw_gizmo(
        Gizmo::new(Vec3::ZERO, 1.0, Color::WHITE)
            .on_click(|| println!("Clicked!"))
            .on_hover(|| println!("Hovered")),
    );

    draw_gizmos_with_line(vec![
        (Vec3::new(2.0, 2.0, 0.0), 0.5),
        (Vec3::new(-2.0, 2.0, 0.0), 0.25),
        (Vec3::new(-2.0, -2.0, 0.0), 0.75),
        (Vec3::new(2.0, -2.0, 0.0), 1.0),
    ]);
}
