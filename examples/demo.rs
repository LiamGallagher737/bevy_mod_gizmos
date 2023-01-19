use bevy::prelude::*;
use bevy_mod_gizmos::*;

#[rustfmt::skip]
fn main() {
    App::new()

        .add_plugins(DefaultPlugins)
        .add_plugin(GizmosPlugin)

        .add_startup_system(setup)
        .add_system(update)

        .init_resource::<GizmosOffset>()
        
        .run();
}

#[derive(Resource, Default)]
struct GizmosOffset(Vec3);

fn setup(mut commands: Commands) {
    println!("Trying hovering and clicking the center gizmo");

    let cam_transform = Transform::from_xyz(0.0, 0.0, 8.0);
    commands.spawn((
        Camera3dBundle {
            transform: cam_transform.looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        GizmoInteractionCamera,
    ));
}

// Notice this resource is read only
fn update(offset: Res<GizmosOffset>) {
    draw_gizmo(
        Gizmo::new(Vec3::ZERO + offset.0, 1.0, Color::WHITE)
            .on_click(|| println!("I've been clicked!"))
            .on_hover(|| println!("Hovered"))
            .on_hover_system(|mut offset: ResMut<GizmosOffset>, time: Res<Time>| {
                offset.0.y -= 0.5 * time.delta_seconds();
            })
            .on_click_system(|mut offset: ResMut<GizmosOffset>| {
                offset.0.y += 0.5;
            }),
    );

    draw_gizmos_with_line(vec![
        (Vec3::new(2.0, 2.0, 0.0) + offset.0, 0.5),
        (Vec3::new(-2.0, 2.0, 0.0) + offset.0, 0.25),
        (Vec3::new(-2.0, -2.0, 0.0) + offset.0, 0.75),
        (Vec3::new(2.0, -2.0, 0.0) + offset.0, 1.0),
    ]);
}
