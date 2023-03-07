# Changelog

> Dates are formatted as DD.MM.YYYY

## Version 0.4.0 (07.03.2023)

- Bevy 0.10 support
- No longer using `lazy_static`

## Version 0.3.1 (19.01.2023)

### Added

- `on_hover_system()` Access any bevy system parameters in a hover interaction
- `on_click_system()` Access any bevy system parameters in a click interaction


## Version 0.3.0 (18.01.2023)

### Added

- Hover gizmo interactions

### Changed

- Much simpler API
- Updated example
- Entire backend

### Fixed

- Framerate was being limited to ~60
- Gizmo flickering
