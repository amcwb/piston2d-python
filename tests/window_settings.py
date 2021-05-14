import piston2d
from piston2d.window import WindowSettings
settings = WindowSettings("test", (100, 100))


print(settings.title)
settings.title = "testing"
print(settings.title)