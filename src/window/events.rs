use graphics::Viewport as PistonViewport;
use pyo3::prelude::*;

use piston::{Button as PistonButton, Event as PistonEvent, EventSettings as PistonEventSettings, Events as PistonEvents, PressEvent, ReleaseEvent, RenderArgs as PistonRenderArgs, RenderEvent, UpdateArgs as PistonUpdateArgs, UpdateEvent};

use crate::input::Button;

use super::Window;

#[pyclass]
pub struct Event {
    pub _piston: PistonEvent,
}

#[pymethods]
impl Event {
    fn is_input(&self) -> PyResult<bool> {
        match &self._piston {
            PistonEvent::Input(..) => Ok(true),
            _ => Ok(false),
        }
    }

    fn is_loop(&self) -> PyResult<bool> {
        match &self._piston {
            PistonEvent::Loop(..) => Ok(true),
            _ => Ok(false),
        }
    }

    fn render_args(&self) -> PyResult<Option<RenderArgs>> {
        match self._piston.render_args() {
            Some(event) => Ok(Some(event.into())),
            None => Ok(None),
        }
    }

    fn update_args(&self) -> PyResult<Option<UpdateArgs>> {
        match self._piston.update_args() {
            Some(event) => Ok(Some(event.into())),
            None => Ok(None),
        }
    }

    fn press_args(&self) -> PyResult<Option<Button>> {
        match self._piston.press_args() {
            Some(event) => Ok(Some(event.into())),
            None => Ok(None),
        }
    }

    fn release_args(&self) -> PyResult<Option<Button>> {
        match self._piston.release_args() {
            Some(event) => Ok(Some(event.into())),
            None => Ok(None),
        }
    }

    // Extra utility
    fn keypress_args(&self) -> PyResult<Option<Button>> {
        match self._piston.press_args() {
            Some(event) => {
                if let PistonButton::Keyboard(_) = event {
                    Ok(Some(event.into()))
                } else {
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }

    fn keyrelease_args(&self) -> PyResult<Option<Button>> {
        match self._piston.release_args() {
            Some(event) => {
                if let PistonButton::Keyboard(_) = event {
                    Ok(Some(event.into()))
                } else {
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }

    fn mousepress_args(&self) -> PyResult<Option<Button>> {
        match self._piston.press_args() {
            Some(event) => {
                if let PistonButton::Mouse(_) = event {
                    Ok(Some(event.into()))
                } else {
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }

    fn mouserelease_args(&self) -> PyResult<Option<Button>> {
        match self._piston.release_args() {
            Some(event) => {
                if let PistonButton::Mouse(_) = event {
                    Ok(Some(event.into()))
                } else {
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }
}


#[pyclass]
pub struct RenderArgs {
    _piston: PistonRenderArgs,
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
            _piston: self._piston.viewport(),
        })
    }
}

impl From<PistonRenderArgs> for RenderArgs {
    fn from(pra: PistonRenderArgs) -> Self {
        RenderArgs { _piston: pra }
    }
}

#[pyclass]
pub struct UpdateArgs {
    pub _piston: PistonUpdateArgs,
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
        UpdateArgs { _piston: pua }
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct Viewport {
    pub _piston: PistonViewport,
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

#[pyclass]
#[derive(Clone, Copy)]
pub struct EventSettings {
    _piston: PistonEventSettings,
}

#[pymethods]
impl EventSettings {
    #[new]
    fn new() -> Self {
        EventSettings {
            _piston: PistonEventSettings::new(),
        }
    }
    #[getter]
    fn max_fps(&self) -> PyResult<u64> {
        Ok(self._piston.max_fps)
    }
    #[getter]
    fn ups(&self) -> PyResult<u64> {
        Ok(self._piston.ups)
    }
    #[getter]
    fn ups_reset(&self) -> PyResult<u64> {
        Ok(self._piston.ups_reset)
    }
    #[getter]
    fn swap_buffers(&self) -> PyResult<bool> {
        Ok(self._piston.swap_buffers)
    }
    #[getter]
    fn bench_mode(&self) -> PyResult<bool> {
        Ok(self._piston.bench_mode)
    }
    #[getter]
    fn lazy(&self) -> PyResult<bool> {
        Ok(self._piston.lazy)
    }
}

#[pyclass]
pub struct Events {
    pub _piston: PistonEvents,
}

/// Basic implementation
#[pymethods]
impl Events {
    #[new]
    fn new(settings: EventSettings) -> Self {
        Events {
            _piston: PistonEvents::new(settings._piston),
        }
    }

    fn next(&mut self, window: &mut Window) -> PyResult<Option<Event>> {
        Ok(match self._piston.next(&mut window._piston) {
            Some(event) => Some(Event { _piston: event }),
            None => None,
        })
    }
}


pub fn init_submodule(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Event>()?;
    m.add_class::<Events>()?;
    m.add_class::<EventSettings>()?;
    m.add_class::<Viewport>()?;
    m.add_class::<RenderArgs>()?;
    m.add_class::<UpdateArgs>()?;

    Ok(())
}