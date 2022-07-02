<div align="center">

# Visual Gizmos for Bevy

Visual gizmos to aid with development and debugging in [Bevy](https://bevyengine.org/)

<!--
Add image/gif here
-->

</div>



# Setup

Add the following to your `Cargo.toml`
```toml
bevy_gizmos = { git = "https://github.com/LiamGallagher737/bevy_gizmos" }
```

Add the plugin to your app
```rs
.add_plugin(bevy_gizmos::GizmosPlugin)
```

To increase performance I recommend the following in your `Cargo.toml`
```toml
[profile.dev.package."*"]
opt-level = 3
```



# Demo
```console
cargo run --example CommingSoon™ 
```



# Basic Use

The following will render a spherical gizmo
```rs
draw_gizmo(SphereGizmo::new(position, diameter, color));
```



# Gizmo Types

### Sphere Gizmo
```rs
struct SphereGizmo {
    position: Vec3,
    diameter: f32,
    color: Color,
}
```
```rs
SphereGizmo::new(position, diameter, color);
```

### Box Gizmo
```rs
struct BoxGizmo {
    position: Vec3,
    scale: Vec3,
    color: Color,
}
```
```rs
BoxGizmo::new(position, scale, color);
```
```rs
BoxGizmo::new_cube(position, size, color);
```

### Mesh Gizmo
```rs
struct MeshGizmo {
    position: Vec3,
    scale: Vec3,
    mesh_handle: Handle<Mesh>,
    color: Color,
}
```
```rs
MeshGizmo::new(position, scale, mmesh_handle, color);
```



# Custom Gizmos

You can create your own custom gizmos by impleenting `bevy_gizmos::Gizmo` on any struct
```rs
impl Gizmo for YourCustomGizmo {
    fn get_transform(&self) -> Transform {
        // Transform
    }

    fn get_color(&self) -> Color {
        // Color
    }

    fn get_mesh_handle(&self) -> Handle<Mesh> {
        // Mesh Handle
    }
}
```
