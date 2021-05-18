use core::f64;
use graphics::Context as PistonContext;
use graphics::{circle_arc as piston_circle_arc, math::Matrix2d, rectangle as piston_rectangle};
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

/// Expand a native rust `[[T; 3]; 2]` into a PyList
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

/// Context of a draw loop
#[pyclass(module = "piston2d.graphics")]
#[derive(Clone)]
pub struct Context {
    pub _piston: PistonContext,
}

#[pymethods]
impl Context {
    /// Get the current viewport
    ///
    /// :type: Optional[Viewport]
    #[getter]
    fn viewport(&self) -> PyResult<Option<Viewport>> {
        Ok(match self._piston.viewport {
            Some(viewport) => Some(Viewport { _piston: viewport }),
            None => None,
        })
    }

    /// The view transformation
    ///
    /// :type: Tuple[Tuple[float, float, float], Tuple[float, float, float]]
    #[getter]
    fn view(&self) -> PyResult<PyObject> {
        Ok(create_matrix2x3_pylist(self._piston.view)?)
    }

    /// Gets the transformation for this context.
    ///
    /// .. note::
    ///
    ///     This should be used as the base context for all objects draw to the
    ///     screen
    ///
    /// :rtype: Tuple[Tuple[float, float, float], Tuple[float, float, float]]
    fn transform(&self) -> PyResult<PyObject> {
        Ok(create_matrix2x3_pylist(self._piston.transform)?)
    }

    // #[getter]
    // fn draw_state(&self) -> PyResult<()> {
    //     Ok(())
    // }

    /// Reset the current transformation to the default
    fn reset(&self) -> PyResult<()> {
        self._piston.reset();

        Ok(())
    }

    /// Store the current transformation as a new view
    fn store_view(&self) -> PyResult<()> {
        self._piston.store_view();

        Ok(())
    }

    /// The current view size
    ///
    /// :type: Tuple[float, float]
    #[getter]
    fn get_view_size(&self) -> PyResult<[f64; 2]> {
        Ok(self._piston.get_view_size())
    }
}

/// rectangle(color, rect, transform, g) -> None
///
/// Draws a rectangle to the Opengl Graphics backend.
///
/// :param color: The RGBA color with values between 0.0 and 1.0
///
/// :type color: Tuple[float, float, float, float]
///
/// :param rect: The rect of the rectangle
///
/// :type rect: Tuple[float, float, float, float]
///
/// :param transform: The color, a list of length 4 that is
///
/// :type transform: Tuple[Tuple[float, float, float], Tuple[float, float, float]]
///
/// :param g: The GlGraphics instance
///
/// :type g: GlGraphics
#[pyfunction(rectangle, module = "piston2d.graphics")]
pub fn rectangle(
    color: [f32; 4],
    rect: [f64; 4],
    transform: Option<&PyList>,
    g: &mut GlGraphics,
) -> PyResult<()> {
    piston_rectangle(color, rect, matrix2x3(transform)?, &mut g._piston);

    Ok(())
}

/// circle_arc(color, radius, start, end, rect, transform, g) -> None
///
/// Draws a circle arc to the Opengl Graphics backend.
///
/// :param radius: The radius of the circle arc
///
/// :type radius: float
///
/// :param start: The starting angle of the circle arc
///
/// :type start: float
///
/// :param end: The end angle of the circle arc
///
/// :type end: float
///
/// :param color: The RGBA color with values between 0.0 and 1.0
///
/// :type color: Tuple[float, float, float, float]
///
/// :param rect: The rect of the rectangle
///
/// :type rect: Tuple[float, float, float, float]
///
/// :param transform: The color, a list of length 4 that is
///
/// :type transform: Tuple[Tuple[float, float, float], Tuple[float, float, float]]
///
/// :param g: The GlGraphics instance
///
/// :type g: GlGraphics
#[pyfunction(circle_arc, module = "piston2d.graphics")]
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
