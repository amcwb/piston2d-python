
import piston2d
from piston2d.window import WindowSettings
from piston2d.window import Window
from piston2d.input import Key

settings = WindowSettings("test", (100, 100))
window = Window(settings)

while True:
    event = window.wait_event()
    if button := event.press_args():
        if button.value == Key.A:
            print("A")

    if args := event.update_args():
        print(args.dt)