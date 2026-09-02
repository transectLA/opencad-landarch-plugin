# Contributing
1. Run `cargo fmt --all`.
2. Run `cargo test -p landarch-core`.
3. Run `cargo check --workspace` against the pinned OpenCADStudio revision.
4. Keep `plugin.toml` and the in-code `MANIFEST` synchronized.
5. Put domain calculations in `landarch-core`; keep host glue in the root crate.
6. New semantic entity types must document their XDATA schema in `PLUGIN.md`.
