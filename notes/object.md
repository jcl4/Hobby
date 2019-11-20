# Object
Objects hold GPU resorces that the user does not have access to create so they are build by the obect builder.

## ObjectBuilder
The object builder follows the typical rust builder pattern.  the build function is passed a reference to the application so it can access the GPU resorces in the Renderer.

## Object Properies
* Mesh
* transform
* update_fn
	- closure passed to object builder and called during game loop used to update the object transform (or potentially any other properties as needed)
* material type
* vertex_buffer
* index_buffer
* bind_group
* uniform buffer


