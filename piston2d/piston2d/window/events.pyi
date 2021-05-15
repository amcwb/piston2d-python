from piston2d.piston2d.window import Window
from typing import Optional, Tuple


class Event:
    def is_input(self) -> bool: ...
    def is_loop(self) -> bool: ...

    def render_args(self) -> Optional[RenderArgs]: ...
    def update_args(self) -> Optional[UpdateArgs]: ...


class RenderArgs:
    @property
    def ext_dt(self) -> float: ...
    @property
    def window_size(self) -> Tuple[float, float]: ...
    @property
    def draw_size(self) -> Tuple[float, float]: ...
    @property
    def viewport(self) -> Viewport: ...

class UpdateArgs:
    @property
    def dt(self) -> float: ...

class Viewport:
    @property
    def rect(self) -> Tuple[int, int]: ...
    @property
    def draw_size(self) -> Tuple[int, int]: ...
    @property
    def window_size(self) -> Tuple[float, float]: ...

class EventSettings:
    def __init__(self) -> None: ...
    @property
    def max_fps(self) -> int: ...
    @property
    def ups(self) -> int: ...
    @property
    def ups_reset(self) -> int: ...
    @property
    def swap_buffers(self) -> bool: ...
    @property
    def bench_mode(self) -> bool: ...
    @property
    def lazy(self) -> bool: ...

class Events:
    def __init__(self, settings: EventSettings) -> None: ...
    def next(self, window: Window) -> Optional[Event]: ...

