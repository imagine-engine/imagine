# =============================================================================
# test_frame.py
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

import os
from imagine import *

def test_frame():
  frame = render()
  pixel = frame[0, 0]

  assert frame.width == 1920
  assert frame.height == 1080
  assert pixel.r == 0
  assert pixel.g == 0
  assert pixel.b == 0
  assert len(frame.row(0)) == 1920
  assert len(frame.column(0)) == 1080

  frame.save()
  assert 'frame.png' in os.listdir()

  frame.save('test.png')
  assert 'test.png' in os.listdir()

  os.remove('frame.png')
  os.remove('test.png')