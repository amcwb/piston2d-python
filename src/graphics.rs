use core::f64;

use graphics::Context as PistonContext;
use pyo3::prelude::*;

use crate::window::events::Viewport;

pub mod rectangle;


#[pyclass]
#[derive(Clone)]
pub struct Context {
    pub _piston: PistonContext
}

#[pymethods]
impl Context {
    #[getter]
    fn viewport(&self) -> PyResult<Option<Viewport>> {
        Ok(match self._piston.viewport {
            Some(viewport) => Some(Viewport { _piston: viewport }),
            None => None
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