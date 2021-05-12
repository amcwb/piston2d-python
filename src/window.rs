use pyo3::prelude::*;

use glutin_window::GlutinWindow;
use piston::{Size, WindowSettings as PistonWindowSettings};

#[pyclass]
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

    fn get_title(&self) -> PyResult<String> {
        Ok(self.__internal.get_title())
    }

    fn set_title(&mut self, title: String) -> PyResult<()> {
        self.__internal.set_title(title);

        Ok(())
    }

    fn get_size(&self) -> PyResult<[u32; 2]> {
        Ok(self.__internal.get_size().into())
    }

    
    fn set_size(&mut self, value: [u32; 2]) -> PyResult<()> {
        self.__internal.set_size(Size::from(value));
        Ok(())
    }

}

#[pyclass(unsendable)]
pub struct Window {
    pub __internal: GlutinWindow
}

#[pymethods]
impl Window {
    #[new]
    fn new(settings: Py<WindowSettings>) -> PyResult<Self> {
        
        Ok(Python::with_gil(|py| {
            let window_settings: WindowSettings = settings.extract(py).unwrap();
            Window {
                __internal: GlutinWindow::new(&window_settings.__internal).unwrap()
            }
        }))
    }
}