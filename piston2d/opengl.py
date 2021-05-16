from .piston2d import opengl
from .graphics import Context
from .window.events import Viewport
from typing import Any, Callable

GlGraphics = opengl.GlGraphics

def draw(gl: GlGraphics, viewport: Viewport, func: Callable[[Context, "GlGraphics"], None]):
    ctx = gl.draw_begin(viewport)

    func(ctx, gl)

    gl.draw_end()