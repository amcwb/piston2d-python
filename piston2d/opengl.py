from .piston2d import opengl
from .graphics import Context
from .window.events import Viewport
from typing import Any, Callable

GlGraphics = opengl.GlGraphics

def draw(gl: GlGraphics, viewport: Viewport, func: Callable[[Context, "GlGraphics"], None]) -> None:
    """
    Wrapper around the :func:`GlGraphics.draw_begin` and
    :func:`GlGraphics.draw_end` pair, similar to the piston2d built-in
    `GlGraphics.draw`.

    :param gl: The graphics instance to use
    :type gl: GlGraphics
    :param viewport: The viewport (to pass to :func:`GlGraphics.draw_begin`)
    :type viewport: Viewport
    :param func: The function to call with the context (:class:`Context`)
    :type func: (p1: :class:`Context`, p2: :class:`GlGraphics`) -> ``None``
    """
    ctx = gl.draw_begin(viewport)

    func(ctx, gl)

    gl.draw_end()