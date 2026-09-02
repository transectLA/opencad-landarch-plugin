# Land Arch Tools v0.0.2

A landscape-architecture add-on for **Open CAD Studio** focused on design and construction-documentation workflows.

**Package:** `opencad-landarch-plugin`  
**Rust cdylib crate:** `landarch`  
**Plugin ID:** `opencad.landarch`  
**Display name:** `Land Arch Tools`  
**Command namespace:** `LA_`

## Architecture

The repository follows the structure proven by the working `opencad-landsurvey-plugin`:

- root `cdylib` = OpenCADStudio glue only;
- `src/ribbon.rs` = ribbon tree;
- `src/dispatch.rs` = command routing;
- `src/state.rs` = process-local plugin state;
- `crates/landarch-core` = host-free domain logic + tests;
- `crates/landarch-cli` = headless catalog inspection;
- XDATA is written through OpenCADStudio host APIs for DWG/DXF persistence.

## Ribbon organization

OpenCADStudio currently exposes **one ribbon tab per plugin**, so the requested second materials/site ribbon is implemented as a dedicated second ribbon area in the **Land Arch Tools** tab:

1. Project / Planting / Plant Layout
2. **Materials / Furnishings / Amenities**
3. Documentation

The site-ribbon functions are isolated in `src/ribbon.rs` and commands in `src/commands/site.rs`, so they can be moved into a second `CadModule` if the host later supports multiple modules per plugin.

## Commands

Planting: `LA_PLANTS`, `LA_PALETTES`, `LA_TREE`, `LA_SHRUB`, `LA_AREA`, `LA_GRID`, `LA_NATURALIZE`, `LA_LABEL`, `LA_PLANT_SCHEDULE`.

Materials/site: `LA_MATERIALS`, `LA_MAT_AREA`, `LA_MAT_EDGE`, `LA_FURNISHINGS`, `LA_FURN`, `LA_AMENITIES`, `LA_AMENITY`, `LA_SITE_SCHEDULE`.

Documentation: `LA_SCHEDULE`, `LA_QC`, `LA_STATUS`, `LA_HELP`.

## Build compatibility

This revision intentionally pins the same OpenCADStudio v0.9.8 / cadcodec dependency pattern used by the working Land Survey plugin instead of tracking `main`.

```bash
cargo check --workspace
cargo test -p landarch-core
cargo build --release
```

The compiled library is `landarch.dll`, `liblandarch.so`, or `liblandarch.dylib` depending on platform.

## Important v0.0.2 limitation

Material area and edge quantities are captured when the semantic object is created. If geometry is manually edited afterward, quantity metadata is not automatically recomputed because OpenCADStudio does not yet expose a general entity-change notification hook to external plugins. `LA_QC` reports this limitation explicitly.
