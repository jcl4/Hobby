# Hobby Library Functionality

## Game
The Game object is the main entry point to the library and configured via the `GameSettings` struct. Game loop lives here

### Creation
``` Rust
let game_settings = GameSettings::defualt();
let app = Application::new(game_settings);

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
