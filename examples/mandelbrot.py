# import imagine
# from imagine import gpu

def mandelbrot():
  z = gpu.vec(0, 0)
  c = gpu.coord - gpu.vec(0.5, 0)
  inside = True

  for i in gpu.range(100):
    z = gpu.vec(z.x^2 - z.y^2, z.x*z.y + z.y*z.x + c)
    if length(z) > 2:
      inside = False
      break

  gpu.color = gpu.white if inside else gpu.black

# fractal = gpu.register(mandelbrot)
# fractal.run(duration=10)