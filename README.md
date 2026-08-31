# Land Arch Tools v0.0.2

Package: `opencad-landarch-plugin`  
Rust crate: `landarch`  
Display name: **Land Arch Tools**  
Command prefix: `LA_`

## Included v0.0.2 features

- plant catalog
- plant palettes
- interactive tree placement
- interactive shrub/grass placement
- rectangular planting-area creation
- regular grid population
- deterministic naturalized population
- LANDARCH XDATA on plant and planting-area objects
- plant labeling
- plant schedule / quantity reporting
- landscape planting layers
- build workflow and manual test plan

## Commands

- `LA_HELP`
- `LA_PLANTS`
- `LA_PALETTES`
- `LA_TREE [CODE]`
- `LA_SHRUB [CODE]`
- `LA_AREA [PALETTE] [SPACING]`
- `LA_GRID CODE X1 Y1 X2 Y2 SPACING`
- `LA_NATURALIZE CODE X1 Y1 X2 Y2 SPACING`
- `LA_LABEL CODE`
- `LA_SCHEDULE`

## Examples

```text
LA_TREE QAGR
LA_SHRUB RHIN
LA_AREA NATIVE 4
LA_GRID LOMI 0 0 60 30 3
LA_NATURALIZE MUCA 0 0 60 30 3
LA_LABEL QAGR
LA_SCHEDULE
```

## Layers

- `L-PLNT-SYMB`
- `L-PLNT-AREA`
- `L-ANNO-PLNT`

## Build

Replace `__RUSTC_VERSION__` in `plugin.toml` with the exact compiler version expected by the target Open CAD Studio build, then:

```bash
cargo build --release
```

Install the compiled dynamic library and `plugin.toml` in the Open CAD Studio plugin directory for `opencad.landarch`.

## Current limitation

Batch population is coordinate-driven because the current interactive plugin callback model does not expose arbitrary document mutation from every interactive point callback. The internal object schema is designed so this can later become fully associative without changing the basic plant model.
