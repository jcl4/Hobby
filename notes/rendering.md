# Rendering Process
## User Process
* Starts with creating a mesh
	- A mesh contains a vector of verticies and a vector of indices
	- see [Verex](vertex.md) for more details on how vertices work
* Objects are created from Meshes
	- Objects are created using the Object Builder Struct
	- Object require GPU resources to draw so can not be built directly by the user 
	- see [Object](object.md) for more details on objects
	- Obects also hold Transforms and [Materials](material.md)
* Ojbects are added to A Scene
	- Scenes are very simple right now and only hold a vector of objects
* Scenes are sent to the application for rendering
	-	Currently only supporting one scene being run
	-	Set up splash screen during this phase?

## Internal Process
* the [Renderer](renderer.md) creates and stores the global GPU resources
	- Needs to be built during app initialization to allow for building objects
	- the renderer holds options of each specific pipeline needed based on objects built
### Drawing
- Renderer loops through each ojbect in the active scene and updates its uniform buffer if needed
- it sets the appropriate pipeline and then calls the draw call of the object

