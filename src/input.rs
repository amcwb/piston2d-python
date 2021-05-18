use pyo3::prelude::*;

use piston::Button as PistonButton;

#[pyclass(module = "piston2d.input")]
pub struct Button {
    pub _piston: PistonButton,
}

#[pymethods]
impl Button {
    /// is_key() -> bool
    ///
    /// Whether the button is a keyboard button
    ///
    /// :rtype: bool
    fn is_key(&self) -> PyResult<bool> {
        match &self._piston {
            PistonButton::Keyboard(..) => Ok(true),
            _ => Ok(false),
        }
    }

    /// is_controller() -> bool
    ///
    /// Whether the button is a controller button
    ///
    /// :rtype: bool
    fn is_controller(&self) -> PyResult<bool> {
        match &self._piston {
            PistonButton::Controller(..) => Ok(true),
            _ => Ok(false),
        }
    }

    /// is_hat() -> bool
    ///
    /// Whether the button is a controller hat (d-pad)
    ///
    /// :rtype: bool
    fn is_hat(&self) -> PyResult<bool> {
        match &self._piston {
            PistonButton::Hat(..) => Ok(true),
            _ => Ok(false),
        }
    }

    /// is_mouse() -> bool
    ///
    /// Whether the button is a mouse button
    ///
    /// :rtype: bool
    fn is_mouse(&self) -> PyResult<bool> {
        match &self._piston {
            PistonButton::Mouse(..) => Ok(true),
            _ => Ok(false),
        }
    }

    /// value() -> Optional[int]
    ///
    /// The "value" of this button, regardless of type
    ///
    /// .. warning::
    ///
    ///     Due to the nature of this function, values for types may collide!
    ///     It is instaed recommended to check the type of the button before
    ///     trying to get its value.
    ///
    /// .. warning::
    ///
    ///     This early implementation of the function returns ``None`` for both
    ///     ``Controller`` and ``Hat`` buttons. This will likely change.
    ///
    /// :rtype: Optional[int]
    fn value(&self) -> PyResult<Option<u32>> {
        Ok(match &self._piston {
            PistonButton::Keyboard(key) => Some((*key).into()),
            PistonButton::Mouse(mouse) => Some((*mouse).into()),

            PistonButton::Controller(_) => None,
            PistonButton::Hat(_) => None,
        })
    }
}

impl From<PistonButton> for Button {
    fn from(pb: PistonButton) -> Self {
        Button { _piston: pb }
    }
}

pub fn init_submodule(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Button>()?;

    Ok(())
}
