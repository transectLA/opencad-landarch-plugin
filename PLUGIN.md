# Land Arch Tools command + XDATA reference

## XDATA applications

- `LANDARCH_PLANT`
- `LANDARCH_PLANTING_AREA`
- `LANDARCH_MATERIAL`
- `LANDARCH_FURNISHING`
- `LANDARCH_AMENITY`

All records begin with `schema=2` and then UTF-8 `key=value` strings.

### Plant
`object_type=plant`, `code`, `botanical`, `common`, `category`, `spacing`, `role`.

### Planting area
`object_type=planting_area`, `area_id`, `palette`, `spacing`, `area`.

### Material
`object_type=material`, `code`, `name`, `category`, `unit`, `quantity`, `spec`, `detail`, `unit_cost`.

### Furnishing / Amenity
`object_type=furnishing|amenity`, `code`, `name`, `manufacturer`, `model`, `unit_cost`, `spec`, `detail`.

## Command examples

```text
LA_TREE QAGR
LA_SHRUB HEAR
LA_AREA NATIVE 4
LA_GRID LOMI 0 0 60 30 3
LA_NATURALIZE MUCA 0 0 60 30 3
LA_MAT_AREA PAV-CONC
LA_MAT_EDGE EDGE-STEEL
LA_FURN BENCH-01
LA_AMENITY DRINK-01
LA_PLANT_SCHEDULE
LA_SITE_SCHEDULE
LA_SCHEDULE
LA_QC
```
