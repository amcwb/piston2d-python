import pyston2d
from pyston2d.window import WindowSettings
settings = WindowSettings("test", (100, 100))

print(pyston2d.__version__)
print(settings.get_title())