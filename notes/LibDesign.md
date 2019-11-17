# Hobby Library Design Document
Planning doc for the library functionality

## Aplication
- Entry Point to Library
- Holds renderer, input, scene, etc
- Game Loop
- Created with a WindowSettings
- factory functionality

### API 
``` Rust
let window_settings = WindowSettings::defualt();
let app = Application::new(window_settings();

let object = ObjectBuilder()
	.with_mesh(mesh)
	.with_transform(transform)
	.with_material(Material::type)
	.build(app);

let scene = Scene::new();
scene.add_object(object);

app.start(scene);

```

## Structs
### Object
- Built using application
- Holds all required buffers
- Checks to make sure that vertex supports the material type

### Scene
- Scene has a vector of objects
- Also creates list of unique materials

- May eventually becoem a scene graph???

### Mesh
- Mesh has vec of Vertices and Indicies

### Vertex
- Containls all the various attributes
- Position only required attribute
- All other attribures are Options
- Needs to align with material type

### Renderer
- Holds all the required rendering data
- Holds pipelines
- Application passes list unique materials to renderer create_pipelines 



### Material
- Enum with different available materials
``` Rust
enum Material {
	SolidColor(Color),
	VertexColor,
	Texture(Texture),
	Phong(PhongData),
}
```
