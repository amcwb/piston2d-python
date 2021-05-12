import pyston2d

app = pyston2d.init("my title", (100, 100))

@app.render
def render():
    print("Hi")

@app.update
def update():
    print("Bye")

while True:
    app.tick()