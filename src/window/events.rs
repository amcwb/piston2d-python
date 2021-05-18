use graphics::Viewport as PistonViewport;
use pyo3::prelude::*;

use piston::{
    Button as PistonButton, Event as PistonEvent, EventSettings as PistonEventSettings,
    Events as PistonEvents, PressEvent, ReleaseEvent, RenderArgs as PistonRenderArgs, RenderEvent,
    UpdateArgs as PistonUpdateArgs, UpdateEvent,
};

use crate::input::Button;

use super::Window;

#[pyclass(module = "piston2d.window.events")]
pub struct Event {
    pub _piston: PistonEvent,
}

#[pymethods]
impl Event {
    /// is_input() -> bool
    ///
    /// Whether the event comes from an ``Event::Input``
    ///
    /// :rtype: bool
    fn is_input(&self) -> PyResult<bool> {
        match &self._piston {
            PistonEvent::Input(..) => Ok(true),
            _ => Ok(false),
        }
    }

    /// is_loop() -> bool
    ///
    /// Whether the event comes from an ``Event::Loop``
    ///
    /// :rtype: bool
    fn is_loop(&self) -> PyResult<bool> {
        match &self._piston {
            PistonEvent::Loop(..) => Ok(true),
            _ => Ok(false),
        }
    }

    /// render_args() -> Optional[RenderArgs]
    ///
    /// The render args for this event, if this is a render event
    ///
    /// :rtype: Optional[RenderArgs]
    fn render_args(&self) -> PyResult<Option<RenderArgs>> {
        match self._piston.render_args() {
            Some(event) => Ok(Some(event.into())),
            None => Ok(None),
        }
    }

    /// update_args() -> Optional[UpdateArgs]
    ///
    /// The update args for this event, if this is an update event
    ///
    /// :rtype: Optional[UpdateArgs]
    fn update_args(&self) -> PyResult<Option<UpdateArgs>> {
        match self._piston.update_args() {
            Some(event) => Ok(Some(event.into())),
            None => Ok(None),
        }
    }

    /// press_args() -> Optional[Button]
    ///
    /// The button (:class:`Key`, :class:`Mouse`) pressed, if this is a button
    /// press event.
    ///
    /// :rtype: Optional[Button]
    fn press_args(&self) -> PyResult<Option<Button>> {
        match self._piston.press_args() {
            Some(event) => Ok(Some(event.into())),
            None => Ok(None),
        }
    }

    /// release_args() -> Optional[Button]
    ///
    /// The button (:class:`Key`, :class:`Mouse`) released, if this is a button
    /// release event.
    ///
    /// :rtype: Optional[Button]
    fn release_args(&self) -> PyResult<Option<Button>> {
        match self._piston.release_args() {
            Some(event) => Ok(Some(event.into())),
            None => Ok(None),
        }
    }

    /// keypress_args() -> Optional[Button]
    ///
    /// Utility function that returns the :class:`Button` for this event,
    /// only if this event is a keypress.
    ///
    /// :rtype: Optional[Button]
    fn keypress_args(&self) -> PyResult<Option<Button>> {
        match self._piston.press_args() {
            Some(event) => {
                if let PistonButton::Keyboard(_) = event {
                    Ok(Some(event.into()))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    /// keyrelease_args() -> Optional[Button]
    ///
    /// Utility function that returns the :class:`Button` for this event,
    /// only if this event is a key release.
    ///
    /// :rtype: Optional[Button]
    fn keyrelease_args(&self) -> PyResult<Option<Button>> {
        match self._piston.release_args() {
            Some(event) => {
                if let PistonButton::Keyboard(_) = event {
                    Ok(Some(event.into()))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    /// mousepress_args() -> Optional[Button]
    ///
    /// Utility function that returns the :class:`Button` for this event,
    /// only if this event is a mouse click.
    ///
    /// :rtype: Optional[Button]
    fn mousepress_args(&self) -> PyResult<Option<Button>> {
        match self._piston.press_args() {
            Some(event) => {
                if let PistonButton::Mouse(_) = event {
                    Ok(Some(event.into()))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    /// mouserelease_args() -> Optional[Button]
    ///
    /// Utility function that returns the :class:`Button` for this event,
    /// only if this event is a mouse release.
    ///
    /// :rtype: Optional[Button]
    fn mouserelease_args(&self) -> PyResult<Option<Button>> {
        match self._piston.release_args() {
            Some(event) => {
                if let PistonButton::Mouse(_) = event {
                    Ok(Some(event.into()))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

/// Render args for an event
#[pyclass(module = "piston2d.window.events")]
pub struct RenderArgs {
    _piston: PistonRenderArgs,
}

#[pymethods]
impl RenderArgs {
    /// The extrapolated delta in seconds. Use to do smooth animation!
    #[getter]
    fn ext_dt(&self) -> PyResult<f64> {
        Ok(self._piston.ext_dt)
    }

    /// The window size/viewport in points
    #[getter]
    fn window_size(&self) -> PyResult<[f64; 2]> {
        Ok(self._piston.window_size)
    }

    /// The draw size
    #[getter]
    fn draw_size(&self) -> PyResult<[u32; 2]> {
        Ok(self._piston.draw_size)
    }

    /// The viewport
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

/// Update args for an event
#[pyclass(module = "piston2d.window.events")]
pub struct UpdateArgs {
    pub _piston: PistonUpdateArgs,
}

#[pymethods]
impl UpdateArgs {
    /// Delta time in seconds
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

/// Information of the viewport
#[pyclass(module = "piston2d.window.events")]
#[derive(Clone, Copy)]
pub struct Viewport {
    pub _piston: PistonViewport,
}

#[pymethods]
impl Viewport {
    /// The viewport in pixels
    #[getter]
    fn rect(&self) -> PyResult<[i32; 4]> {
        Ok(self._piston.rect)
    }

    /// The draw size
    #[getter]
    fn draw_size(&self) -> PyResult<[u32; 2]> {
        Ok(self._piston.draw_size)
    }

    /// The window size/viewport in points
    #[getter]
    fn window_size(&self) -> PyResult<[f64; 2]> {
        Ok(self._piston.window_size)
    }
}

/// Settings for the :class:`Events` utility
#[pyclass(module = "piston2d.window.events")]
#[derive(Clone, Copy)]
pub struct EventSettings {
    _piston: PistonEventSettings,
}

#[pymethods]
impl EventSettings {
    /// EventSettings() -> EventSettings
    ///
    /// Create the default settings
    #[new]
    fn new() -> Self {
        EventSettings {
            _piston: PistonEventSettings::new(),
        }
    }

    /// Max FPS
    #[getter]
    fn max_fps(&self) -> PyResult<u64> {
        Ok(self._piston.max_fps)
    }

    /// Number of updates per second
    #[getter]
    fn ups(&self) -> PyResult<u64> {
        Ok(self._piston.ups)
    }

    /// Number of delayed updates to display before skipping them to catch up
    #[getter]
    fn ups_reset(&self) -> PyResult<u64> {
        Ok(self._piston.ups_reset)
    }

    /// Whether display buffers are automatically swapped
    #[getter]
    fn swap_buffers(&self) -> PyResult<bool> {
        Ok(self._piston.swap_buffers)
    }

    /// Whether we are in benchmark mode
    #[getter]
    fn bench_mode(&self) -> PyResult<bool> {
        Ok(self._piston.bench_mode)
    }

    /// Whether this event retriever emits update and idle events
    #[getter]
    fn lazy(&self) -> PyResult<bool> {
        Ok(self._piston.lazy)
    }
}

/// Events utility that provides helper functions for handling the event loop.
#[pyclass(module = "piston2d.window.events")]
pub struct Events {
    pub _piston: PistonEvents,
}

/// Basic implementation
#[pymethods]
impl Events {
    /// Events(settings: EventSettings) -> Events
    ///
    /// Create a new event loop handler
    #[new]
    #[args(settings)]
    fn new(settings: EventSettings) -> Self {
        Events {
            _piston: PistonEvents::new(settings._piston),
        }
    }

    /// next(window: Window) -> Optional[Event]
    ///
    /// Get the next event
    ///
    /// :param window: The window to run on
    /// :type window: Window
    /// :returns: Next event or ``None`` (when ending event loop)
    /// :rtype: Optional[Event]
    #[args(window)]
    fn next(&mut self, window: &mut Window) -> PyResult<Option<Event>> {
        Ok(match self._piston.next(&mut window._piston) {
            Some(event) => Some(Event { _piston: event }),
            None => None,
        })
    }
}

pub fn init_submodule(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Event>()?;
    m.add_class::<Events>()?;
    m.add_class::<EventSettings>()?;
    m.add_class::<Viewport>()?;
    m.add_class::<RenderArgs>()?;
    m.add_class::<UpdateArgs>()?;

    Ok(())
}
