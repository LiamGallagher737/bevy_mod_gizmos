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
bevy_mod_gizmos = { git = "https://github.com/LiamGallagher737/bevy_mod_gizmos" }
```

Add this to any file you want to use gizmos in
```rs
use bevy_mod_gizmos::*:
```

Add the plugin to your app
```rs
.add_plugin(bevy_mod_gizmos::GizmosPlugin)
```

For interactive gizmos add the following when creating your camera
```rs
.insert_bundle(GizmoInteractionCamera::default())
```

To increase performance I recommend the following in your `Cargo.toml`
```toml
[profile.dev.package."*"]
opt-level = 3
```



<!--
# Demo
```console
cargo run --example CommingSoon™ 
```
-->



# Usage

Render a single gizmo
```rs
draw_gizmo(gizmo);
```

Render multiple gizmos and a connecting line
```rs
draw_gizmos(vec![gizmo, gizmo, gizmo], true);
```

Render a line
```rs
draw_line(points, color);
```



# Gizmos

```rs
Gizmo::sphere(position, size, color)
```
```rs
Gizmo::cube(position, size, color)
```
```rs
Gizmo::box(position, scale, color)
```
```rs
Gizmo::capsule(position, width, height, color)
```
```rs
Gizmo::torus(position, size, color)
```
```rs
Gizmo::new(position, scale, color, mesh_handle)
```



# License

This project is licensed under the [MIT license](https://github.com/LiamGallagher737/bevy_gizmos/blob/main/LICENSE). Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in bevy_mod_gizmos by you, shall be licensed as MIT, without any additional terms or conditions.
