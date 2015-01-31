
extern crate sdl2;
extern crate "sdl2-sys" as sdl2_sys;

// Re-export some of the symbols in event.rs
pub use event::{KeyCode,MouseButton,Event};
pub use shape::{Rect,Point,Polygon};
pub use window::Window;

mod event;
mod shape;
mod window;
