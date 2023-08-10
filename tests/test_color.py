# =============================================================================
# test_color.py
# =============================================================================
# Copyright 2023 Menelik Eyasu

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

def test_color():
  black = Color.BLACK
  white = Color.WHITE
  blue = Color.BLUE

  assert black.r == 0
  assert black.g == 0
  assert black.b == 0

  assert white.r == 255
  assert white.g == 255
  assert white.b == 255

  assert blue.r == 33
  assert blue.g == 150
  assert blue.b == 243

  assert black.__repr__() == '#000000'
  assert white.__repr__() == '#FFFFFF'
  assert blue.__repr__() == '#2196F3'

  assert black == black
  assert white == white
  assert blue == blue

  assert black != white
  assert white != blue
  assert blue != black