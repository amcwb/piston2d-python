from .piston2d import graphics

__doc__ = graphics.__doc__

Context = graphics.Context
rectangle = graphics.rectangle
circle_arc = graphics.circle_arc

# Due to limitations of pyo3, we must set modules manually
rectangle.__module__ = "piston2d.graphics"
circle_arc.__module__ = "piston2d.graphics"