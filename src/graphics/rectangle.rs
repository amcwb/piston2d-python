use std::{rc::Rc, sync::Arc};

use graphics::{math::identity, rectangle as piston_rectangle};
use pyo3::{prelude::*, wrap_pyfunction};

use crate::opengl::GlGraphics;


#[pyfunction]
pub fn rectangle(color: [f32; 4], rect: [f64; 4], g: Py<GlGraphics>) -> PyResult<()> {
    Python::with_gil(|py| {
        let gl: GlGraphics = g.extract(py).unwrap();
        let mut _piston = &mut*gl._piston.lock().unwrap();
        piston_rectangle(color, rect, Default::default(), _piston);
        drop(_piston);
    });

    Ok(())
}

pub fn init_submodule(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rectangle, m)?)?;
    
    Ok(())
}