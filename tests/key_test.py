
from piston2d.window.events import Events, EventSettings
from piston2d.window import WindowSettings
from piston2d.window import Window

window = Window(WindowSettings("test", (100, 100)))
events = Events(EventSettings())

keys = []

while event := events.next(window):
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