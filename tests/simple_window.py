
from piston2d.graphics import circle_arc, rectangle
from piston2d.graphics import Context
from piston2d.window.events import Events, EventSettings
from piston2d.window import WindowSettings
from piston2d.window import Window
from piston2d.opengl import GlGraphics, draw
import random
import math

window = Window(WindowSettings("test", (180, 180)))
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
        # Begin the draw loop
        context = graphics.draw_begin(args.viewport)
        window.title = f"{red():.2f} {green():.2f} {blue():.2f}"

        # Make background
        graphics.clear_color([red(), green(), blue(), 1.0])

        # Draw circle arc in center (ish)
        circle_arc([1 - red(), 1 - green(), 1 - blue(), 1.0], 15.0, 0.0,
                   math.tau, [50, 50, 80, 80], context.transform(), graphics)

        # Draw rectangle in top left corner
        rectangle([1 - red(), 1 - green(), 1 - blue(), 1.0],
                  [0, 0, 30, 30], context.transform(), graphics)
        
        # End the draw loop
        graphics.draw_end()
