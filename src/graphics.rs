use core::f64;
use graphics::{Rectangle, circle_arc as piston_circle_arc, rectangle as piston_rectangle};
use graphics::Context as PistonContext;
use pyo3::{prelude::*, wrap_pyfunction};

use crate::{opengl::GlGraphics, window::events::Viewport};

#[pyclass]
#[derive(Clone)]
pub struct Context {
    pub _piston: PistonContext,
}

#[pymethods]
impl Context {
    #[getter]
    fn viewport(&self) -> PyResult<Option<Viewport>> {
        Ok(match self._piston.viewport {
            Some(viewport) => Some(Viewport { _piston: viewport }),
            None => None,
        })
    }
    // #[getter]
    // fn view(&self) -> PyResult<[[f64; 3]; 2]> {
    //     Ok(self._piston.view)
    // }
    // #[getter]
    // fn transform(&self) -> PyResult<[[f64; 3]; 2]> {
    //     Ok(self._piston.transform)
    // }
    // #[getter]
    // fn draw_state(&self) -> PyResult<()> {
    //     Ok(())
    // }
    fn reset(&self) -> PyResult<()> {
        self._piston.reset();

        Ok(())
    }
    fn store_view(&self) -> PyResult<()> {
        self._piston.store_view();

        Ok(())
    }
    #[getter]
    fn get_view_size(&self) -> PyResult<[f64; 2]> {
        Ok(self._piston.get_view_size())
    }
}

#[pyfunction]
pub fn rectangle(color: [f32; 4], rect: [f64; 4], g: &mut GlGraphics) -> PyResult<()> {
    piston_rectangle(color, rect, Default::default(), &mut g._piston);

    Ok(())
}

#[pyfunction]
pub fn circle_arc(color: [f32; 4], radius: f64, start: f64, end: f64, rect: [f64; 4],  g: &mut GlGraphics) -> PyResult<()> {
    piston_circle_arc(color, radius, start, end, rect, Default::default(), &mut g._piston);

    Ok(())
}

pub fn init_submodule(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Context>()?;
    m.add_function(wrap_pyfunction!(rectangle, m)?)?;
    m.add_function(wrap_pyfunction!(circle_arc, m)?)?;

    Ok(())
}
