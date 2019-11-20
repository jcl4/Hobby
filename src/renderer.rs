/*
Renderer workflow thoughts
- Scene holds a vector of models and a collection of unique materials
- a Material enum is exposed to the user to select various materials
- actual materials are rendering pipelines
- Renderer holds GPU objects
- Object hold all its required buffers and a draw call
- 
- Object builder builds the buffers and bind group
- to build the bind group the bind group layout will need to be created
*/

use winit::window::Window;

/// The renderer is responsible for displaying a scene on screan
///
/// It holds all required GPU resources
pub(crate) struct Renderer{}

impl Renderer {
	pub(crate) fn new(window: &Window) -> Renderer {
		Renderer{}
	}

	pub(crate) fn render(&self) {}
}
