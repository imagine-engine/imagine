# =============================================================================
# test_main_scene.py
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

def test_default_main_scene():
  assert main_scene.duration == 0
  assert main_scene.lights == []
  assert main_scene.objects == []

def test_wait():
  assert main_scene.duration == 0
  wait(13)
  assert main_scene.duration == 13
  wait(5)
  assert main_scene.duration == 18