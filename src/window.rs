use pyo3::prelude::*;

use glutin_window::GlutinWindow;
use piston::{Size, WindowSettings as PistonWindowSettings};

#[pyclass(module="piston2d")]
pub struct WindowSettings {
    pub __internal: PistonWindowSettings
}

impl Clone for WindowSettings {
    fn clone(&self) -> Self {
        WindowSettings {
            __internal: self.__internal.clone()
        }
    }
}

#[pymethods]
impl WindowSettings {
    #[new]
    fn new(title: String, size: [u32; 2]) -> Self {
        WindowSettings {
            __internal: PistonWindowSettings::new(title, size)
        }
    }

    #[getter]
    fn get_title(&self) -> PyResult<String> {
        Ok(self.__internal.get_title())
    }

    #[setter]
    fn set_title(&mut self, title: String) -> PyResult<()> {
        self.__internal.set_title(title);
        Ok(())
    }

    #[getter]
    fn get_size(&self) -> PyResult<[u32; 2]> {
        Ok(self.__internal.get_size().into())
    }
    
    #[setter]
    fn set_size(&mut self, value: [u32; 2]) -> PyResult<()> {
        self.__internal.set_size(Size::from(value));
        Ok(())
    }

    #[getter]
    fn get_fullscreen(&self) -> PyResult<bool> {
        Ok(self.__internal.get_fullscreen())
    }

    #[setter]
    fn set_fullscreen(&mut self, value: bool) -> PyResult<()> {
        self.__internal.set_fullscreen(value);
        Ok(())
    }
    
    #[getter]
    fn get_exit_on_esc(&self) -> PyResult<bool> {
        Ok(self.__internal.get_exit_on_esc()) 
    }

    #[setter]
    fn set_exit_on_esc(&mut self, value: bool) -> PyResult<()> {
        self.__internal.set_exit_on_esc(value);
        Ok(())
    }

    #[getter]
    fn get_automatic_close(&self) -> PyResult<bool> {
        Ok(self.__internal.get_automatic_close())
    }

    #[setter]
    fn set_automatic_close(&mut self, value: bool) -> PyResult<()> {
        self.__internal.set_automatic_close(value);
        Ok(())
    }

    #[getter]
    fn get_samples(&self) -> PyResult<u8> {
        Ok(self.__internal.get_samples())
    }

    #[setter]
    fn set_samples(&mut self, value: u8) -> PyResult<()> {
        self.__internal.set_samples(value);
        Ok(())
    }

    #[getter]
    fn get_vsync(&self) -> PyResult<bool> {
        Ok(self.__internal.get_vsync())
    }

    #[setter]
    fn set_vsync(&mut self, value: bool) -> PyResult<()> {
        self.__internal.set_vsync(value);
        Ok(())
    }

    #[getter]
    fn get_srgb(&self) -> PyResult<bool> {
        Ok(self.__internal.get_srgb())
    }

    #[setter]
    fn set_srgb(&mut self, value: bool) -> PyResult<()> {
        self.__internal.set_srgb(value);
        Ok(())
    }

    #[getter]
    fn get_resizable(&self) -> PyResult<bool> {
        Ok(self.__internal.get_resizable())
    }

    #[setter]
    fn set_resizable(&mut self, value: bool) -> PyResult<()> {
        self.__internal.set_resizable(value);
        Ok(())
    }

    #[getter]
    fn get_decorated(&self) -> PyResult<bool> {
        Ok(self.__internal.get_decorated())
    }

    #[setter]
    fn set_decorated(&mut self, value: bool) -> PyResult<()> {
        self.__internal.set_decorated(value);
        Ok(())
    }

    #[getter]
    fn get_controllers(&self) -> PyResult<bool> {
        Ok(self.__internal.get_controllers())
    }

    #[setter]
    fn set_controllers(&mut self, value: bool) -> PyResult<()> {
        self.__internal.set_controllers(value);
        Ok(())
    }

    #[getter]
    fn get_transparent(&self) -> PyResult<bool> {
        Ok(self.__internal.get_transparent())
    }

    #[setter]
    fn set_transparent(&mut self, value: bool) -> PyResult<()> {
        self.__internal.set_transparent(value);
        Ok(())
    }
}

#[pyclass(unsendable, module="piston2d")]
pub struct Window {
    pub __internal: GlutinWindow
}

#[pymethods]
impl Window {
    #[new]
    fn new(settings: Py<WindowSettings>) -> PyResult<Self> {
        // Extract value with the GIL to use it to construct a new window
        Ok(Python::with_gil(|py| {
            let window_settings: WindowSettings = settings.extract(py).unwrap();
            Window {
                __internal: GlutinWindow::new(&window_settings.__internal).unwrap()
            }
        }))
    }
}