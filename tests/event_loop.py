import piston2d
from piston2d.window import WindowSettings
from piston2d.window import Window

settings = WindowSettings("test", (100, 100))
window = Window(settings)

while True:
    event = window.wait_event()
    print(event.render_args())
    print(event.update_args())