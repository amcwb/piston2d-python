# piston2d
[![](https://img.shields.io/github/v/release/starsflower/piston2d-python.svg?include_prereleases)](https://GitHub.com/starsflower/piston2d-python/releases/) ![](https://img.shields.io/github/license/starsflower/piston2d-python.svg) [![made-with-python](https://img.shields.io/badge/Made%20with-Python-1f425f.svg)](https://www.python.org/) [![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-b7410e.svg)](https://www.rust-lang.org/) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-green.svg)](https://github.com/starsflower/piston2d-python/pulls)




`piston2d`, or `piston2d-python`, is a Python binding for the amazing Rust library `piston2d`.

*This is very WIP, so I will not accept any issues (yet). Assume that breaking changes happen with every commit.*

## Getting started
This assumes you have installed the library to your computer. This is not meant to be extensive, but will develop into a proper tutorial as the project goes on.

### Setting up the window and event handlers
```python
from piston2d.window import Window
from piston2d.window import WindowSettings

from piston2d.window.events import Events
from piston2d.window.events import EventSettings

window = Window(WindowSettings("my window name", (180, 180)))
events = Events(EventSettings())
```

### Setting up graphics
```python
from piston2d.opengl import GlGraphics

# ...

graphics = GlGraphics("3.2")
```

### Start the loop
```python
# List to keep track of keys...
keys = []

while event := events.next(window):
    if button := event.press_args():
        keys.append(button.value())
        print("Keys pressed: {}".format(keys))

    if button := event.release_args():
        try:
            keys.remove(button.value())
        except:
            pass
```

### Draw loop
```python
while event := events.next(window):
    # ...
    if args := event.render_args():
        # Begin the draw loop
        context = graphics.draw_begin(args.viewport)

        # Draw code here
        
        # End the draw loop
        graphics.draw_end()
```

### Drawing shapes
As of writing, only the rectangle and circle arc are implemented:

```python
from piston2d.graphics import circle_arc, rectangle

WHITE = [1.0, 1.0, 1.0, 1.0]
# ...
while event := events.next(window):
    # ...
    if args := event.render_args():
        # Begin the draw loop
        context = graphics.draw_begin(args.viewport)

        # Draw circle arc in center (ish)
        circle_arc(WHITE, 15.0, 0.0,
                   math.tau, [50, 50, 80, 80], context.transform(), graphics)

        # Draw rectangle in top left corner
        rectangle(WHITE,
                  [0, 0, 30, 30], context.transform(), graphics)
        
        # End the draw loop
        graphics.draw_end()
```

You may also want to clear the screen:
```python
# ...

BLACK = [0.0, 0.0, 0.0, 1.0]
while event := events.next(window):
    # ...
    if args := event.render_args():
        # Begin the draw loop
        context = graphics.draw_begin(args.viewport)

        # Clear the screen
        graphics.clear_color(BLACK)

        # Draw code here...
        
        # End the draw loop
        graphics.draw_end()
```

## Examples
- `tests/simple_window.py` &bull; A simple window to test drawing using `GlGraphics`.

  Example:
  ![Example](https://i.discord.fr/Vyv.png)

## Building
Use setuptools to build
```python
pip install setuptools wheel setuptools_rust
python setup.py install
```
