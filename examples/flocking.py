# import imagine
# from imagine import gpu

class FlockingShader:
  class Agent:
    position: gpu.vec
    rotation: float

  def main(agents: list[Agent]):
    pass

flocking = gpu.register(FlockingShader)
flocking.run(duration=10)