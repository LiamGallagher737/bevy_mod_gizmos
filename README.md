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
Gizmo::cubiod(position, scale, color)
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

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
