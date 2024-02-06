# =============================================================================
# test_path.py
# =============================================================================
# Copyright 2024 Menelik Eyasu

# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at

#     http://www.apache.org/licenses/LICENSE-2.0

# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
# =============================================================================

from imagine import *
from pytest import approx
from imagine.objects import Square, Rectangle, Triangle

def test_bounds():
  square1 = Square()
  assert square1.bounds[0] == approx(-0.5)
  assert square1.bounds[1] == approx(-0.5)
  assert square1.bounds[2] == approx(0.5)
  assert square1.bounds[3] == approx(0.5)

  square2 = Square(size=5.0)
  assert square2.bounds[0] == approx(-2.5)
  assert square2.bounds[1] == approx(-2.5)
  assert square2.bounds[2] == approx(2.5)
  assert square2.bounds[3] == approx(2.5)

  rect1 = Rectangle()
  assert rect1.bounds[0] == approx(-1.0)
  assert rect1.bounds[1] == approx(-0.5)
  assert rect1.bounds[2] == approx(1.0)
  assert rect1.bounds[3] == approx(0.5)

  rect2 = Rectangle(width=3.0, height=2.0)
  assert rect2.bounds[0] == approx(-1.5)
  assert rect2.bounds[1] == approx(-1.0)
  assert rect2.bounds[2] == approx(1.5)
  assert rect2.bounds[3] == approx(1.0)