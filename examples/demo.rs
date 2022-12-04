use bevy::prelude::*;
use bevy_mod_gizmos::*;

#[rustfmt::skip]
fn main() {
    App::new()

        .add_plugins(DefaultPlugins)
        .add_plugin(GizmosPlugin)

        .add_startup_system(setup)
        .add_system(move_entities)
        .add_system(gizmos)
        
        .run();
}

fn gizmos(query: Query<&Transform, Without<Camera>>) {
    let positions: Vec<Vec3> = query.iter().map(|t| t.translation).collect();
    draw_gizmos(
        vec![
            Gizmo::sphere(positions[0], 1.0, Color::GREEN).on_click(|_| println!("Clicked Sphere")),
            Gizmo::cube(positions[1], 1.0, Color::RED).on_click(|_| println!("Clicked Cube")),
            Gizmo::cubiod(positions[2], Vec3::new(1.0, 0.5, 1.5), Color::BLUE)
                .on_click(|_| println!("Clicked Cubiod")),
            Gizmo::capsule(positions[3], 1.0, 1.5, Color::ORANGE)
                .on_click(|_| println!("Clicked Capsule")),
            Gizmo::torus(positions[4], 1.0, Color::YELLOW).on_click(|_| println!("Clicked Torus")),
        ],
        false, // Draw a line?
    );
}

fn setup(mut commands: Commands) {
    // Spawn camera
    let cam_transform = Transform::from_xyz(4.0, 5.0, 8.0);
    commands.spawn((
        Camera3dBundle {
            transform: cam_transform.looking_at([4.0, 0.0, 0.0].into(), Vec3::Y),
            ..Default::default()
        },
        GizmoInteractionCamera::default(),
    ));

    // Create one entity for each gizmo type
    for i in 0..5 {
        commands.spawn(TransformBundle::from_transform(Transform::from_xyz(
            i as f32 * 2.0,
            0.0,
            0.0,
        )));
    }
}

fn move_entities(mut query: Query<&mut Transform, Without<Camera>>, time: Res<Time>) {
    for (i, mut transform) in query.iter_mut().enumerate() {
        transform.translation.y = (time.elapsed_seconds() + i as f32).sin();
    }
}
