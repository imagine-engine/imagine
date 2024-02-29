from imagine import *
from imagine.objects import Circle
from imagine.animation import interpolate

record()

x, y, z = 0.01, 0.0, 0.0
a, b, c = 10.0, 28.0, 8.0 / 3.0

dt = 0.01

title = Text('The Lorenz Attractor', size=20)

for frame in interpolate(duration=30.0):
  # px, py, pz = x, y, z
  x += dt * (a * (y - x))
  y += dt * (x * (b - z) - y)
  z += dt * (x * y - c * z)
  # path.add_line(3*px, 3*px, 3*x, 3*y)

  point = Circle(radius=1)
  point.position.x = 3 * x
  point.position.y = 3 * y

stop()