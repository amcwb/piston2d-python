use std::time::Duration;

use pyo3::prelude::*;

use glutin_window::GlutinWindow;
use piston::{AdvancedWindow, OpenGLWindow, Size, Window as PistonWindow, WindowSettings as PistonWindowSettings};

use self::events::Event;

pub mod events;

#[pyclass(module="piston2d")]
pub struct WindowSettings {
    pub _piston: PistonWindowSettings
}

impl Clone for WindowSettings {
    fn clone(&self) -> Self {
        WindowSettings {
            _piston: self._piston.clone()
        }
    }
}

#[pymethods]
impl WindowSettings {
    #[new]
    fn new(title: String, size: [u32; 2]) -> Self {
        WindowSettings {
            _piston: PistonWindowSettings::new(title, size)
        }
    }

    #[getter]
    fn get_title(&self) -> PyResult<String> {
        Ok(self._piston.get_title())
    }

    #[setter]
    fn set_title(&mut self, title: String) -> PyResult<()> {
        self._piston.set_title(title);
        Ok(())
    }

    #[getter]
    fn get_size(&self) -> PyResult<[u32; 2]> {
        Ok(self._piston.get_size().into())
    }
    
    #[setter]
    fn set_size(&mut self, value: [u32; 2]) -> PyResult<()> {
        self._piston.set_size(Size::from(value));
        Ok(())
    }

    #[getter]
    fn get_fullscreen(&self) -> PyResult<bool> {
        Ok(self._piston.get_fullscreen())
    }

    #[setter]
    fn set_fullscreen(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_fullscreen(value);
        Ok(())
    }
    
    #[getter]
    fn get_exit_on_esc(&self) -> PyResult<bool> {
        Ok(self._piston.get_exit_on_esc()) 
    }

    #[setter]
    fn set_exit_on_esc(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_exit_on_esc(value);
        Ok(())
    }

    #[getter]
    fn get_automatic_close(&self) -> PyResult<bool> {
        Ok(self._piston.get_automatic_close())
    }

    #[setter]
    fn set_automatic_close(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_automatic_close(value);
        Ok(())
    }

    #[getter]
    fn get_samples(&self) -> PyResult<u8> {
        Ok(self._piston.get_samples())
    }

    #[setter]
    fn set_samples(&mut self, value: u8) -> PyResult<()> {
        self._piston.set_samples(value);
        Ok(())
    }

    #[getter]
    fn get_vsync(&self) -> PyResult<bool> {
        Ok(self._piston.get_vsync())
    }

    #[setter]
    fn set_vsync(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_vsync(value);
        Ok(())
    }

    #[getter]
    fn get_srgb(&self) -> PyResult<bool> {
        Ok(self._piston.get_srgb())
    }

    #[setter]
    fn set_srgb(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_srgb(value);
        Ok(())
    }

    #[getter]
    fn get_resizable(&self) -> PyResult<bool> {
        Ok(self._piston.get_resizable())
    }

    #[setter]
    fn set_resizable(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_resizable(value);
        Ok(())
    }

    #[getter]
    fn get_decorated(&self) -> PyResult<bool> {
        Ok(self._piston.get_decorated())
    }

    #[setter]
    fn set_decorated(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_decorated(value);
        Ok(())
    }

    #[getter]
    fn get_controllers(&self) -> PyResult<bool> {
        Ok(self._piston.get_controllers())
    }

    #[setter]
    fn set_controllers(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_controllers(value);
        Ok(())
    }

    #[getter]
    fn get_transparent(&self) -> PyResult<bool> {
        Ok(self._piston.get_transparent())
    }

    #[setter]
    fn set_transparent(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_transparent(value);
        Ok(())
    }
}

#[pyclass(unsendable, module="piston2d")]
pub struct Window {
    pub _piston: GlutinWindow,

    // To aid cloning
    pub _settings: WindowSettings
}

impl Clone for Window {
    fn clone(&self) -> Self {
        Window {
            _piston: GlutinWindow::new(&self._settings._piston).unwrap(),
            _settings: self._settings.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

/// Implements AdvancedWindow
#[pymethods]
impl Window {
    #[new]
    fn new(settings: Py<WindowSettings>) -> PyResult<Self> {
        // Extract value with the GIL to use it to construct a new window
        Ok(Python::with_gil(|py| {
            let window_settings: WindowSettings = settings.extract(py).unwrap();
            Window {
                _piston: GlutinWindow::new(&window_settings._piston).unwrap(),
                _settings: window_settings
            }
        }))
    }

    #[getter]
    fn get_title(&self) -> PyResult<String> {
        Ok(self._piston.get_title())
    }

    #[setter]
    fn set_title(&mut self, title: String) -> PyResult<()> {
        self._piston.set_title(title);
        Ok(())
    }

    #[getter]
    fn get_size(&self) -> PyResult<[u32; 2]> {
        Ok(self._piston.size().into())
    }
    
    #[setter]
    fn set_size(&mut self, value: [u32; 2]) -> PyResult<()> {
        self._piston.set_size(Size::from(value));
        Ok(())
    }

    #[getter]
    fn get_exit_on_esc(&self) -> PyResult<bool> {
        Ok(self._piston.get_exit_on_esc()) 
    }

    #[setter]
    fn set_exit_on_esc(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_exit_on_esc(value);
        Ok(())
    }

    #[getter]
    fn get_automatic_close(&self) -> PyResult<bool> {
        Ok(self._piston.get_automatic_close())
    }

    #[setter]
    fn set_automatic_close(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_automatic_close(value);
        Ok(())
    }
    
    #[getter]
    fn get_position(&self) -> PyResult<Option<(i32, i32)>> {
        let position = self._piston.get_position();
        
        Ok(match position {
            Some(pos) => Some(pos.into()),
            None => Option::None
        })
    }
    
    #[setter]
    fn set_position(&mut self, position: (i32, i32)) -> PyResult<()> {
        self._piston.set_position(position);

        Ok(())
    }

    #[setter]
    fn set_capture_cursor(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_capture_cursor(value);
        Ok(())
    }

    fn show(&mut self) -> PyResult<()> {
        self._piston.show();

        Ok(())
    }

    fn hide(&mut self) -> PyResult<()> {
        self._piston.hide();

        Ok(())
    }
}

/// Implements OpenGLWindow
#[pymethods]
impl Window {
    fn is_current(&self) -> PyResult<bool> {
        Ok(self._piston.is_current())
    }

    fn make_current(&mut self) -> PyResult<()> {
        self._piston.make_current();
        
        Ok(())
    }
}

// Implements Window
#[pymethods]
impl Window {
    #[getter]
    fn draw_size(&self) -> PyResult<(f64, f64)> {
        Ok(self._piston.draw_size().into())
    }

    #[getter]
    fn get_should_close(&self) -> PyResult<bool> {
        Ok(self._piston.should_close())
    }

    #[setter]
    fn set_should_close(&mut self, value: bool) -> PyResult<()> {
        self._piston.set_should_close(value);

        Ok(())
    }

    fn swap_buffers(&mut self) -> PyResult<()> {
        self._piston.swap_buffers();

        Ok(())
    }

    fn wait_event(&mut self) -> PyResult<Event> {
        Ok(Event {
            _piston: self._piston.wait_event()
        })
    }

    fn wait_event_timeout(&mut self, seconds: f64) -> PyResult<Option<Event>> {
        Ok(match self._piston.wait_event_timeout(Duration::from_secs_f64(seconds)) {
            Some(event) => Some(Event {
                _piston: event
            }),
            _ => None
        })
    }
}