# Hobby Library Functionality

## Overview
The Application object is the main entry point to the library and configured via the `AppSettings` struct. Game loop lives here

### Creation
``` Rust
let app_settings = AppSettings::defualt();
let app = Application::new(app_settings);

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
### Application
#### Properties
- Window
- Renderer
- InputState

#### Methods
`new(app_settings) -> Application`
- Creats window, renderer, and InputState

`run(self, scene)`
- pass list of unique pipelines to renderer
- renderer creates and holds onto pipelines

### Object
#### Properties
- Mesh
- Material
- IndexBuffer
- VertexBuffer
- UniformBuffer
- UpdateFunction

#### Methods
`update(&mut self)`

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
- Application passes list of unique materials to renderer create_pipelines 



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

### Frame Timer
#### Needs
- prints to command line number of frames and average frame rate
- interval for printing is configurable - defined in ApplicationSettings
- prints to command line at end of program

