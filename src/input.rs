use pyo3::prelude::*;

use piston::Button as PistonButton;


#[pyclass]
pub struct Button {
    pub _piston: PistonButton,
}

#[pymethods]
impl Button {
    fn is_key(&self) -> PyResult<bool> {
        match &self._piston {
            PistonButton::Keyboard(..) => Ok(true),
            _ => Ok(false),
        }
    }

    fn is_controller(&self) -> PyResult<bool> {
        match &self._piston {
            PistonButton::Controller(..) => Ok(true),
            _ => Ok(false),
        }
    }

    fn is_hat(&self) -> PyResult<bool> {
        match &self._piston {
            PistonButton::Hat(..) => Ok(true),
            _ => Ok(false),
        }
    }

    fn is_mouse(&self) -> PyResult<bool> {
        match &self._piston {
            PistonButton::Mouse(..) => Ok(true),
            _ => Ok(false),
        }
    }

    fn value(&self) -> PyResult<Option<u32>> {
        Ok(match &self._piston {
            PistonButton::Keyboard(key) => Some((*key).into()),
            PistonButton::Mouse(mouse) => Some((*mouse).into()),
            
            PistonButton::Controller(_) => None,
            PistonButton::Hat(_) => None
        })
    }
}


impl From<PistonButton> for Button {
    fn from(pb: PistonButton) -> Self {
        Button { _piston: pb }
    }
}