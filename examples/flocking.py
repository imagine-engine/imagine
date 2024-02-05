from imagine import *
from imagine.math import dist, Vector

camera.record('flocking.mp4')

N = 100
seperation = 1.5
alignment = 1
cohesion = 1

flock = [Boid() for _ in range(N)]
for _ in animation.interpolate(duration=5):
  for i in range(N):
    c = Vector()
    avg_pos = Vector()
    avg_vel = Vector()

    for j in range(N):
      if (i != j):
        avg_pos += b.position
        avg_vel += b.velocity
        if (dist(flock[i].position, flock[j].position) < 100):
          c -= flock[j].position - flock[i].position

    avg_pos /= N - 1
    avg_vel /= N - 1

    flock[i].velocity += avg_pos + c + avg_vel

camera.stop()