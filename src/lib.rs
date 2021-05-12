extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
use std::{collections::HashSet};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{AdvancedWindow, RenderEvent, UpdateEvent, WindowSettings as PistonWindowSettings};
use pyo3::{prelude::*, wrap_pymodule};
use pyo3::wrap_pyfunction;
use piston::{
    event_loop::{EventSettings, Events},
    Key,
};

mod window;
use window::{WindowSettings, Window};

static version: &str = "0.1.0";

#[pyclass(unsendable)]
struct Pyston2dApp {
    gl: GlGraphics, // OpenGL drawing backend.
    window: GlutinWindow,
    keys: HashSet<Key>,
    events: Events,

    render_handlers: Vec<PyObject>,
    update_handlers: Vec<PyObject>
}

#[pymethods]
impl Pyston2dApp {
    pub fn tick(&mut self) -> PyResult<()> {
        let e = self.events.next(&mut self.window).unwrap();

        if let Some(args) = e.render_args() {
            // self.update(&args);
            let render_handlers = &self.render_handlers;
            self.gl.draw(args.viewport(), |c, gl| {
                for function in render_handlers {
                    Python::with_gil(|py| {
                        function.call0(py);
                    })
                }
            })
        }

        if let Some(args) = e.update_args() {
            for handler in &self.update_handlers {
                Python::with_gil(|py| {
                    handler.call0(py);
                })
            }
        }

        Ok(())
    }

    pub fn render(&mut self, callable: PyObject) -> PyResult<()> {
        Python::with_gil(|py| {
            let function = callable.as_ref(py);
            if function.is_callable() {
                self.render_handlers.push(callable);
            }
        });

        Ok(())
    }
    
    pub fn update(&mut self, callable: PyObject) -> PyResult<()> {
        Python::with_gil(|py| {
            let function = callable.as_ref(py);
            if function.is_callable() {
                self.update_handlers.push(callable);
            }
        });

        Ok(())
    }

    pub fn set_title(&mut self, title: String) -> PyResult<()> {
        self.window.set_title(title);

        Ok(())
    }

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
    let mut window: GlutinWindow = PistonWindowSettings::new(title, dimensions)
        .graphics_api(opengl)
        .decorated(true)
        .exit_on_esc(true)
        .build()
        .unwrap();


    Ok(Pyston2dApp {
        gl: GlGraphics::new(opengl),
        window: window,
        keys: HashSet::new(),
        events: Events::new(EventSettings::new()),
        
        render_handlers: vec![],
        update_handlers: vec![]
    })
}


#[pymodule]
pub fn window(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WindowSettings>()?;
    m.add_class::<Window>()?;

    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyston2d(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_class::<Pyston2dApp>()?;
    
    m.add_wrapped(wrap_pymodule!(window))?;

    m.add("__version__", version)?;
    Ok(())
}