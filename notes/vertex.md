# Vertex
The public Verext struct contains a mandatory 3 element position and all the rest are options

## Properteis
- pos: [f32; 3],
- color: Option<[f32; 4]>,
- uv: Option<[f32; 2]
- normals: Option<[f32; 3]>,

## Notes
- Vertex properties need to align with [Material](material.md) types. 
- Each Material Type has a Vertex Defined with it.
- When objects are built it will check that the required properties are included in the vertex 
- After that the Vertex vector will be turned into the appropriate matearial vertex vector 
