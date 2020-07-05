use ash::extensions::khr::{Surface, XlibSurface};

pub fn required_extension_names() -> Vec<*const i8> {
    vec![Surface::name().as_ptr(), XlibSurface::name().as_ptr()]
}
