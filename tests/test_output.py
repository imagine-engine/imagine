# =============================================================================
# test_output.py
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

def test_record():
  assert output.width == 1920
  assert output.height == 1080
  assert not output.recording

  video_made = os.path.isfile('video.mp4')
  assert not video_made

  record()
  assert output.recording
  assert video_made != os.path.isfile('video.mp4')

  stop()
  assert not output.recording

  labeled_video_made = os.path.isfile('test.mp4')
  assert not labeled_video_made

  record('test.mp4')
  assert output.recording
  assert labeled_video_made != os.path.isfile('test.mp4')

  stop()
  assert not output.recording