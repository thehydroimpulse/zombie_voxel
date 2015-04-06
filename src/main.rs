extern crate glutin_window;
extern crate piston;
extern crate shader_version;

use glutin_window::GlutinWindow;
use piston::window::{WindowSettings, Size};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let version = shader_version::OpenGL::_3_2;
    let window = GlutinWindow::new(version, WindowSettings::new("foobar".to_string(), Size { width: 800, height: 350 })
                              .exit_on_esc(true));

    let window = Rc::new(RefCell::new(window));
    for e in piston::events(window) {
        use piston::event::*;
    }
    println!("Hello, world!");
}
