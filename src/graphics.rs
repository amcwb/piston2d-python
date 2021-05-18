use core::f64;
use graphics::Context as PistonContext;
use graphics::{
    circle_arc as piston_circle_arc, math::Matrix2d, rectangle as piston_rectangle,
};
use pyo3::{prelude::*, types::PyList, wrap_pyfunction};
use std::{convert::TryInto, fmt::Debug};

use crate::{opengl::GlGraphics, window::events::Viewport};

type PyMatrix = PyList;

/// Expand a PyList into a Vec<T>
fn expand<'source, T: Clone + FromPyObject<'source>>(row: &'source PyMatrix) -> PyResult<Vec<T>>
where
    T: 'source,
{
    let mut res: Vec<T> = vec![];
    for item in row.iter() {
        let value = item.extract()?;
        res.push(value)
    }

    Ok(res)
}

/// Expand a PyList into a native rust [[T; 3]; 2]
fn expand2x3<'source, T: Clone + Debug + FromPyObject<'source>>(
    matrix: &'source PyMatrix,
) -> PyResult<[[T; 3]; 2]>
where
    T: 'source,
{
    Ok([
        expand::<T>(matrix.get_item(0).cast_as::<PyList>()?)?
            .try_into()
            .unwrap(),
        expand::<T>(matrix.get_item(1).cast_as::<PyList>()?)?
            .try_into()
            .unwrap(),
    ])
}

/// Create a Matrix2d from a PyList using expand2x3
fn matrix2x3(transform: Option<&PyList>) -> PyResult<Matrix2d> {
    Ok(match transform {
        Some(t) => Matrix2d::from(expand2x3(t)?),
        None => Matrix2d::default(),
    })
}

/// Expand a native rust [[T; 3]; 2] into a PyList
///
/// Return type is specifically vague to avoid fighting with the GIL.
fn create_matrix2x3_pylist<T: ToPyObject + Clone>(transform: [[T; 3]; 2]) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let r1 = PyList::new(py, transform[0].to_vec());
    let r2 = PyList::new(py, transform[1].to_vec());
    let list = PyList::new(py, [r1, r2].to_vec());

    Ok(list.to_object(py))
}

#[pyclass(module="piston2d")]
#[derive(Clone)]
pub struct Context {
    pub _piston: PistonContext,
}

#[pymethods(module="piston2d")]
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
    fn transform(&self) -> PyResult<PyObject> {
        Ok(create_matrix2x3_pylist(self._piston.transform)?)
    }
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

#[pyfunction(module="piston2d.graphics")]
pub fn rectangle(
    color: [f32; 4],
    rect: [f64; 4],
    transform: Option<&PyList>,
    g: &mut GlGraphics,
) -> PyResult<()> {
    piston_rectangle(color, rect, matrix2x3(transform)?, &mut g._piston);

    Ok(())
}

#[pyfunction(module="piston2d.graphics")]
pub fn circle_arc(
    color: [f32; 4],
    radius: f64,
    start: f64,
    end: f64,
    rect: [f64; 4],
    transform: Option<&PyList>,
    g: &mut GlGraphics,
) -> PyResult<()> {
    piston_circle_arc(
        color,
        radius,
        start,
        end,
        rect,
        matrix2x3(transform)?,
        &mut g._piston,
    );

    Ok(())
}

pub fn init_submodule(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Context>()?;
    m.add_function(wrap_pyfunction!(rectangle, m)?)?;
    m.add_function(wrap_pyfunction!(circle_arc, m)?)?;

    Ok(())
}
