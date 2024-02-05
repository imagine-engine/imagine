# =============================================================================
# conftest.py
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
import glob
import shutil
import subprocess

def pytest_sessionstart(session):
    lib = glob.glob('imagine.*.so')
    if lib:
        shutil.move(lib[0], f'tests/imagine.so')
    else:
        subprocess.call(['cargo', 'build'])
        shutil.move(glob.glob('target/debug/*.dylib')[0], 'tests/imagine.so')

def pytest_sessionfinish(session, exitstatus):
    if glob.glob('target'):
        shutil.rmtree('target')
    lib = glob.glob('imagine.*.so')
    if lib:
        os.remove(lib)