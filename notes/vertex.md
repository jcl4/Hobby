# Vertex
The public Verext struct contains a mandatory 3 element position and all the rest are options

## Properteis
- Pos: [f32; 3],
- Color: Option<[f32, 4]>,
- ...

## Notes
Vertex properties need to align with [Material](material.md) types. When objects are built it will check that the required properties are included in the vertex and specific vertices will be built appropriate for the material.