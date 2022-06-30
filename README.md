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

Thats all you need to get gizmos setup!



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
SphereGizmo {
    position: Vec3,
    diameter: f32,
    color: Color,
}
```

### Cube Gizmo
```rs
CubeGizmo {
    position: Vec3,
    size: f32,
    color: Color,
}
```

### Box Gizmo
```rs
BoxGizmo {
    position: Vec3,
    width: f32,
    height: f32,
    depth: f32,
    color: Color,
}
```

### Mesh Gizmo
```rs
MeshGizmo {
    position: Vec3,
    scale: Vec3,
    mesh_handle: Handle<Mesh>,
    color: Color,
}
```



# License
This project is licensed under the [MIT license](https://github.com/LiamGallagher737/bevy_gizmos/blob/main/LICENSE). Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
