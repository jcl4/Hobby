//! A hobby game engine for learning Rust and Game Engine Design
//!
//! # Getting started
//! 
//! [`Application`] is the entry point to the library
//! ```rust,no_run
//! fn main() {
//!		let app_settings = AppSettings::defualt();
//!		let app = Application::new(app_settings);
//!
//!		let object = ObjectBuilder()
//!			.with_mesh(mesh)
//!			.with_transform(transform)
//!			.with_material(Material::type)
//!			.build(app);
//!		
//!		let scene = Scene::new();
//!		scene.add_object(object);
//!		
//!		app.start(scene);
//!	}
//!```
//! [`Application`]: application/struct.Application.html
//! ['ApplicationSettings`]: application/strct.ApplicaitonSettings.html
#![warn(clippy::all)]

mod application;
mod scene;
mod input;

pub(crate) mod tools;
pub(crate) mod renderer;

pub use application::{Application, ApplicationSettings};
pub use scene::{Scene, Object, ObjectBuilder, Mesh};
pub use input::InputState;