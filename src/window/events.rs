use graphics::{Viewport as PistonViewport};
use pyo3::prelude::*;

use piston::{Event as PistonEvent, RenderArgs as PistonRenderArgs, UpdateArgs as PistonUpdateArgs, RenderEvent, UpdateEvent};

#[pyclass]
pub struct Event {
    pub _piston: PistonEvent
}

#[pymethods]
impl Event {
    fn is_input(&self) -> PyResult<bool> {
        match &self._piston {
            PistonEvent::Input(..) => Ok(true),
            _ => Ok(false)
        }
    }

    fn is_loop(&self) -> PyResult<bool> {
        match &self._piston {
            PistonEvent::Loop(..) => Ok(true),
            _ => Ok(false)
        }
    }

    fn render_args(&self) -> PyResult<Option<RenderArgs>> {
        match self._piston.render_args() {
            Some(event  ) => Ok(Some(event.into())),
            None => Ok(None)
        }
    }

    fn update_args(&self) -> PyResult<Option<UpdateArgs>> {
        match self._piston.update_args() {
            Some(event) => Ok(Some(event.into())),
            None => Ok(None)
        }
    }
}

#[pyclass]
pub struct RenderArgs {
    _piston: PistonRenderArgs
}

#[pymethods]
impl RenderArgs {
    #[getter]
    fn ext_dt(&self) -> PyResult<f64> {
        Ok(self._piston.ext_dt)
    }

    #[getter]
    fn window_size(&self) -> PyResult<[f64; 2]> {
        Ok(self._piston.window_size)
    }

    #[getter]
    fn draw_size(&self) -> PyResult<[u32; 2]> {
        Ok(self._piston.draw_size)
    }

    #[getter]
    fn viewport(&self) -> PyResult<Viewport> {
        Ok(Viewport {
            _piston: self._piston.viewport()
        })
    }
}


impl From<PistonRenderArgs> for RenderArgs {
    fn from(pra: PistonRenderArgs) -> Self {
        RenderArgs {
            _piston: pra
        }
    }
}

#[pyclass]
pub struct UpdateArgs {
    _piston: PistonUpdateArgs
}

#[pymethods]
impl UpdateArgs {
    #[getter]
    fn dt(&self) -> PyResult<f64> {
        Ok(self._piston.dt)
    }
}

impl From<PistonUpdateArgs> for UpdateArgs {
    fn from(pua: PistonUpdateArgs) -> Self {
        UpdateArgs {
            _piston: pua
        }
    }
}

#[pyclass]
pub struct Viewport {
    _piston: PistonViewport,
}

#[pymethods]
impl Viewport {
    #[getter]
    fn rect(&self) -> PyResult<[i32; 4]> {
        Ok(self._piston.rect)
    }
    #[getter]
    fn draw_size(&self) -> PyResult<[u32; 2]> {
        Ok(self._piston.draw_size)
    }
    #[getter]
    fn window_size(&self) -> PyResult<[f64; 2]> {
        Ok(self._piston.window_size)
    }
}