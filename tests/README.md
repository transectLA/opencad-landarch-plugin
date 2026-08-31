# Manual Test Plan — Land Arch Tools v0.0.2

1. Build against the exact Open CAD Studio API/toolchain.
2. Install the plugin.
3. Confirm the ribbon title is `Land Arch Tools`.
4. Run `LA_HELP`.
5. Run `LA_PLANTS`.
6. Run `LA_PALETTES`.
7. Run `LA_TREE QAGR` and place a plant.
8. Run `LA_SHRUB RHIN` and place a plant.
9. Run `LA_AREA NATIVE 4` and define two opposite corners.
10. Run `LA_GRID LOMI 0 0 30 20 3`.
11. Run `LA_NATURALIZE MUCA 0 0 30 20 3`.
12. Run `LA_LABEL QAGR`.
13. Run `LA_SCHEDULE`.
14. Save/reopen and verify `LANDARCH` XDATA persistence.
15. Verify layers `L-PLNT-SYMB`, `L-PLNT-AREA`, and `L-ANNO-PLNT`.
