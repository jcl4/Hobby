use winit::window::Window;

/// The renderer is responsible for displaying a scene on screan
///
/// It holds all required GPU resources
pub(crate) struct Renderer{}

impl Renderer {
	pub(crate) fn new(window: &Window) -> Renderer{
		Renderer{}
	}

	pub(crate) fn render(&self) {}
}
