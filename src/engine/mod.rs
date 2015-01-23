extern crate sdl2;
extern crate gl;
use std;

pub enum Event {
    Update,
    Draw,
    KeyPressed,
    KeyReleased,
}

pub struct Engine {
    //window:     sdl2::video::Window,
    renderer:   sdl2::render::Renderer,
}

impl Engine {
    pub fn new(width: isize, height: isize) -> Engine {
        sdl2::init(sdl2::INIT_EVERYTHING);

        gl::load_with(|s| unsafe {
            std::mem::transmute(sdl2::video::gl_get_proc_address(s))
        });

        let window = sdl2::video::Window::new(
            "November Squad",
            sdl2::video::WindowPos::PosCentered,
            sdl2::video::WindowPos::PosCentered,
            width, height,
            sdl2::video::OPENGL,
        ).unwrap();

        let renderer = sdl2::render::Renderer::from_window(
            window,
            sdl2::render::RenderDriverIndex::Auto,
            sdl2::render::ACCELERATED,
        ).unwrap();

        Engine{renderer: renderer}
    }

    pub fn run(&self) {
        loop {
            // Handle events
            match sdl2::event::poll_event() {
                sdl2::event::Event::Quit(_) => break,
                sdl2::event::Event::KeyDown(_, _, key, _, _, _) => {
                    if key == sdl2::keycode::KeyCode::Escape {
                        break;
                    }
                },
                _ => (),
            };
        }
    }

    pub fn quit(&self) {
        sdl2::quit();
    }
}

