use graphics::Graphics;
use opengl_graphics::{GlGraphics as PistonGlGraphics, OpenGL as PistonOpenGL};
use pyo3::prelude::*;
use std::str::FromStr;

use crate::{graphics::Context, window::events::Viewport};

#[pyclass(module="piston2d.opengl")]
pub struct GlGraphics {
    pub _piston: PistonGlGraphics,
}

// This does not implement draw. Instead Python handles that
#[pymethods]
impl GlGraphics {
    #[new]
    fn new(opengl: &str) -> Self {
        let opengl = PistonOpenGL::from_str(opengl).unwrap();

        GlGraphics {
            _piston: PistonGlGraphics::new(opengl),
        }
    }

    fn clear_color(&mut self, color: [f32; 4]) -> PyResult<()> {
        self._piston.clear_color(color);

        Ok(())
    }

    fn clear_stencil(&mut self, value: u8) -> PyResult<()> {
        self._piston.clear_stencil(value);

        Ok(())
    }

    fn draw_begin(&mut self, viewport: Viewport) -> PyResult<Context> {
        let ctx = self._piston.draw_begin(viewport._piston);

        Ok(Context { _piston: ctx })
    }

    fn draw_end(&mut self) -> PyResult<()> {
        self._piston.draw_end();

        Ok(())
    }

    // fn draw(&mut self, viewport: Viewport, f: PyObject) -> PyResult<()> {
    //     self._piston.draw(viewport._piston, |c, gl| {
    //         Python::with_gil(|py| {
    //             let f: &PyAny = f.as_ref(py);
    //             if f.is_callable() {
    //                 f.call1((self,));
    //             }
    //         })
    //     });

    //     Ok(())
    // }
}


pub fn init_submodule(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GlGraphics>()?;

    Ok(())
}
