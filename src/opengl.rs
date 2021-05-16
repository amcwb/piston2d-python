use std::{str::FromStr, sync::{Arc, Mutex}};

use graphics::Graphics;
use opengl_graphics::{GlGraphics as PistonGlGraphics, OpenGL as PistonOpenGL};
use pyo3::prelude::*;

use crate::{graphics::Context, window::events::Viewport};

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct GlGraphics {
    pub _piston: Arc<Mutex<PistonGlGraphics>>
}


// This does not implement draw. Instead Python handles that
#[pymethods]
impl GlGraphics {
    #[new]
    fn new(opengl: &str) -> Self {
        let opengl = PistonOpenGL::from_str(opengl).unwrap();

        GlGraphics {
            _piston: Arc::new(Mutex::new(PistonGlGraphics::new(opengl)))
        }
    }

    fn clear_color(&mut self, color: [f32; 4]) -> PyResult<()> {
        let mut _piston = self._piston.lock().unwrap();
        _piston.clear_color(color);
        drop(_piston);

        Ok(())
    }

    fn clear_stencil(&mut self, value: u8) -> PyResult<()> {
        let mut _piston = self._piston.lock().unwrap();
        _piston.clear_stencil(value);
        drop(_piston);

        Ok(())
    }

    fn draw_begin(&mut self, viewport: Viewport) -> PyResult<Context> {
        let mut _piston = self._piston.lock().unwrap();
        let ctx = _piston.draw_begin(viewport._piston);
        drop(_piston);
        
        Ok(Context {
            _piston: ctx
        })
    }

    fn draw_end(&mut self) -> PyResult<()> {
        let mut _piston = self._piston.lock().unwrap();
        _piston.draw_end();
        drop(_piston);

        Ok(())
    }

    // fn draw(&mut self, viewport: Viewport, f: PyAny) -> PyResult<()> {
    //     let mut _piston = self._piston.lock().unwrap();
    //     _piston.draw(viewport._piston, |c, gl| {
    //         Python::with_gil(|py| {
    //             if f.is_callable() {
    //                 f.call1((self,));
    //             }
    //         })
    //     });

    //     Ok(())
    // }
}