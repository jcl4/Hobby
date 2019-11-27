//! A hobby game engine for learning Rust and Game Engine Design
//!
//! # Getting started
//! 
//! [`Application`] is the entry point to the library
//!
//! [`Application`]: application/struct.Application.html
//! ['ApplicationSettings`]: application/strct.ApplicaitonSettings.html
#![warn(clippy::all)]

mod application;
mod input;

pub(crate) mod tools;
pub(crate) mod renderer;

pub mod math;
pub mod scene;

pub use application::{Application, ApplicationSettings};
pub use input::InputState;