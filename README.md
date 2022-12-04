<div align="center">

# Visual Gizmos for Bevy

Visual gizmos to aid with development and debugging in [Bevy](https://bevyengine.org/)

[<img alt="build status" src="https://img.shields.io/github/workflow/status/LiamGallagher737/bevy_mod_gizmos/CI/main?style=for-the-badge" height="24">](https://github.com/LiamGallagher737/bevy_mod_gizmos/actions)
[<img alt="github" src="https://img.shields.io/badge/github-bevy__mod__gizmos-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="24">](https://github.com/LiamGallagher737/bevy_mod_gizmos)
[<img alt="crates.io" src="https://img.shields.io/crates/v/bevy_mod_gizmos.svg?style=for-the-badge&color=fc8d62&logo=rust" height="24">](https://crates.io/crates/bevy_mod_gizmos)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bevy__mod__gizmos-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="24">](https://docs.rs/bevy_mod_gizmos)

</div>



# Setup

Add the following to your `Cargo.toml`
```toml
bevy_mod_gizmos = "0.2.0"
```

Add this to any file you want to use gizmos in
```rs
use bevy_mod_gizmos::*:
```

Add the plugin to your app
```rs
.add_plugin(GizmosPlugin)
```

For interactive gizmos add the following bundle when spawning your camera
```rs
GizmoInteractionCamera::default()
```

To increase performance I recommend the following in your `Cargo.toml`
```toml
[profile.dev.package."*"]
opt-level = 3
```



# Demo

This exampels showcases all built-in gizmmo types and interactions. Click on a gizmo and it will print to the console its name.

```
cargo run --example demo 
```



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

[More Info](https://docs.rs/bevy_mod_gizmos/latest/bevy_mod_gizmos/gizmo/struct.Gizmo.html)



# Bevy Tracking

|Bevy|bevy_mod_gizmos|
|---|---|
|0.9|0.2.0|
|0.7|0.1.1|



# License

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
