
from piston2d.window import events
import piston2d
from piston2d.window import WindowSettings
from piston2d.window import Window
from piston2d.input import Key

settings = WindowSettings("test", (100, 100))
window = Window(settings)
evt = events.Events(events.EventSettings())

keys = []

while True:
    event = evt.next(window)

    if event is None:
        break

    if button := event.press_args():
        keys.append(button.value())
    
    if button := event.release_args():
        try:
            keys.remove(button.value())
        except:
            pass

    if args := event.update_args():
        pass

    print("Keys pressed: {}".format(keys))