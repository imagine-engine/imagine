from imagine import *
from imagine.objects.vehicles import Car

camera.record('driving.mp4')

car = Car()
racetrack = load_environment('racetrack')

camera.stop()