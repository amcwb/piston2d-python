extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{AdvancedWindow, WindowSettings};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass(unsendable)]
struct Pyston2dApp {
    gl: GlGraphics, // OpenGL drawing backend.
    window: Window
}

impl Pyston2dApp {
    pub fn set_size(&mut self, dimensions: [u32; 2]) -> PyResult<()> {
        self.window.set_size(dimensions);

        Ok(())
    }
}

#[pyfunction]
fn init(title: &str, dimensions: [u32; 2]) -> PyResult<Pyston2dApp> {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new(title, dimensions)
        .graphics_api(opengl)
        .decorated(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    Ok(Pyston2dApp {
        gl: GlGraphics::new(opengl),
        window: window
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyston2d(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;

    Ok(())
}