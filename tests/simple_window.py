
from piston2d.graphics.rectangle import rectangle
from piston2d.graphics import Context
from piston2d.window.events import Events, EventSettings
from piston2d.window import WindowSettings
from piston2d.window import Window
from piston2d.opengl import GlGraphics, draw
import random

window = Window(WindowSettings("test", (100, 100)))
events = Events(EventSettings())
graphics = GlGraphics("3.2")


def change_factory():
    current = 0.01
    change = 0.005
    def inner():
        nonlocal current, change
        if random.random() < 0.6:
            if current <= 0 or current >= 1.0:
                change = -change
            
            current = current + change
        return current
    
    return inner
         
keys = []
red = change_factory()
green = change_factory()
blue = change_factory()


while event := events.next(window):
    if button := event.press_args():
        keys.append(button.value())
        print("Keys pressed: {}".format(keys))
    
    if button := event.release_args():
        try:
            keys.remove(button.value())
        except:
            pass

    if args := event.render_args():
        context = graphics.draw_begin(args.viewport)
        # graphics.clear_color([red(), green(), blue(), 1.0])
        rectangle([1.0, 1.0, 1.0, 1.0], args.viewport.rect, graphics)
        graphics.draw_end()
        pass
