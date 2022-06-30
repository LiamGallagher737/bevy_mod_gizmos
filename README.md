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

Thats all you need to setup gizmos!



# Demo
```console
cargo run --example CommingSoon™ 
```



# Basic Use

To draw a sphere gizmo for a single frame use
```rs
bevy_gizmos::draw_gizmo(SphereGizmo::new(position, diameter, color));
```

To add a persistent sphere gizmo use
```rs
bevy_gizmos::add_gizmo(SphereGizmo::new(position, diameter, color));
```
Both return a `GizmoKey` which can be used to remove the gizmo
```rs
bevy_gizmos::remove_gizmo(key);
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
