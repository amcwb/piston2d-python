use graphics::Graphics;
use opengl_graphics::{GlGraphics as PistonGlGraphics, OpenGL as PistonOpenGL};
use pyo3::prelude::*;
use std::str::FromStr;

use crate::{graphics::Context, window::events::Viewport};

/// ``GlGraphics`` implementation and bindings.
#[pyclass(module = "piston2d.opengl")]
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

    /// clear_color(color)
    ///
    /// Clears the screen with a certain color
    ///
    /// :param color: The color to use
    ///
    /// :type color: [Tuple[float, float, float, float]]
    fn clear_color(&mut self, color: [f32; 4]) -> PyResult<()> {
        self._piston.clear_color(color);

        Ok(())
    }

    /// clear_stencil(value)
    ///
    /// Clears the stencil buffer
    ///
    /// :param value: The value to use
    ///
    /// :type value: int
    fn clear_stencil(&mut self, value: u8) -> PyResult<()> {
        self._piston.clear_stencil(value);

        Ok(())
    }

    /// draw_begin(viewport: Viewport) -> Context
    ///
    /// Setup that should be called at the start of a frame's draw call.
    ///
    /// :param viewport: The viewport (usually collected from `Event.render_args`)
    ///
    /// :type viewport: Viewport
    ///
    /// :returns: The current context
    ///
    /// :rtype: Context
    fn draw_begin(&mut self, viewport: Viewport) -> PyResult<Context> {
        let ctx = self._piston.draw_begin(viewport._piston);

        Ok(Context { _piston: ctx })
    }

    /// draw_end()
    ///
    /// Finalize draw calls.
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

pub fn init_submodule(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GlGraphics>()?;

    Ok(())
}
