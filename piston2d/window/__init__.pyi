from typing import Any, Dict, Optional, Tuple
from . import events

class Window:
    def __init__(self, settings: WindowSettings) -> None: ...
    @property
    def title(self) -> str: ...
    @title.setter
    def title(self, value: str) -> None: ...
    @property
    def size(self) -> Tuple[int, int]: ...
    @size.setter
    def size(self, value: Tuple[int, int]) -> None: ...
    @property
    def fullscreen(self) -> bool: ...
    @fullscreen.setter
    def fullscreen(self, value: bool) -> None: ...
    @property
    def exit_on_esc(self) -> bool: ...
    @exit_on_esc.setter
    def exit_on_esc(self, value: bool) -> None: ...
    @property
    def automatic_close(self) -> bool: ...
    @automatic_close.setter
    def automatic_close(self, value: bool) -> None: ...
    @property
    def position(self) -> Optional[Tuple[int, int]]: ...
    @property.setter
    def position(self, position: Tuple[int, int]): ...
    @property
    def capture_cursor(self, value: bool): ...
    def show(self): ...
    def hide(self): ...
    def is_current(self) -> bool: ...
    def make_current(self): ...
    @property
    def draw_size(self) -> Tuple[float, float]: ...
    @property
    def should_close(self) -> bool: ...
    @should_close.setter
    def should_Close(self, value: bool): ...
    def swap_buffers(self): ...
    def wait_event(self) -> events.Event: ...
    def wait_event_timeout(self, seconds: float) -> Optional[events.Event]: ...

class WindowSettings:
    def __init__(self, title: str, size: Tuple[int, int], **kwargs: Dict[str, Any]) -> None: ...

    @property
    def title(self) -> str: ...
    @title.setter
    def title(self, value: str) -> None: ...
    @property
    def size(self) -> Tuple[int, int]: ...
    @size.setter
    def size(self, value: Tuple[int, int]) -> None: ...
    @property
    def fullscreen(self) -> bool: ...
    @fullscreen.setter
    def fullscreen(self, value: bool) -> None: ...
    @property
    def exit_on_esc(self) -> bool: ...
    @exit_on_esc.setter
    def exit_on_esc(self, value: bool) -> None: ...
    @property
    def automatic_close(self) -> bool: ...
    @automatic_close.setter
    def automatic_close(self, value: bool) -> None: ...
    @property
    def samples(self) -> int: ...
    @samples.setter
    def samples(self, value: int) -> None: ...
    @property
    def srgb(self) -> bool: ...
    @srgb.setter
    def srgb(self, value: bool) -> None: ...
    @property
    def vsync(self) -> bool: ...
    @vsync.setter
    def vsync(self, value: bool) -> None: ...
    @property
    def resizable(self) -> bool: ...
    @resizable.setter
    def resizable(self, value: bool) -> None: ...
    @property
    def decorated(self) -> bool: ...
    @decorated.setter
    def decorated(self, value: bool) -> None: ...
    @property
    def controllers(self) -> bool: ...
    @controllers.setter
    def controllers(self, value: bool) -> None: ...