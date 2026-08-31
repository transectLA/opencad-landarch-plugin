# Land Arch Tools (opencad-landarch-plugin)

Landscape architecture design and construction-document tools for
[Open CAD Studio](https://github.com/HakanSeven12/OpenCADStudio).

- **Package:** `opencad-landarch-plugin`
- **Crate / library name:** `landarch`
- **Command prefix:** `LA_`
- **Plugin id:** `opencad.landarch`

## Status

Bootstrap stage — currently ships a single placeholder command (`LA_HELLO`)
to confirm the plugin loads and dispatches correctly. Real landscape tools
land next; see [PLUGIN.md](./PLUGIN.md) for the planned command list and
XDATA schema.

## Build

```
cargo build --release
```

This produces:

- Linux: `target/release/liblandarch.so`
- Windows: `target/release/landarch.dll`
- macOS: `target/release/liblandarch.dylib`

## Install (local dev)

Copy the built library and `plugin.toml` into your OCS plugins folder:

```
<config>/OpenCADStudio/plugins/opencad.landarch/
  plugin.toml
  liblandarch.so   (or the .dll / .dylib for your platform)
```

Where `<config>` is:

- Windows: `%APPDATA%`
- macOS: `~/Library/Application Support`
- Linux: `$XDG_CONFIG_HOME` or `~/.config`

Restart Open CAD Studio. A **Land Arch** ribbon tab should appear with a
**Hello** button; clicking it should print "Land Arch Tools is alive." to
the command line.

## Roadmap

1. `LA_HELLO` — confirm the plugin loads and dispatches. (done)
2. Plant palette + XDATA schema for tagged plant entities.
3. `LA_TAG` — assign a palette entry (botanical/common name, size, spacing)
   to a selected plant block insert.
4. `LA_SCHEDULE` — scan tagged entities, aggregate quantities by species,
   and insert a formatted Plant Schedule table.
