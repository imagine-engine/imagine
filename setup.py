# =============================================================================
# setup.py
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

import subprocess
from setuptools_rust import Binding, RustExtension
from setuptools import setup, Command, find_namespace_packages

setup(
    name='imagine',
    version='0.1',
    author='TheNeuronalCoder',
    license='apache-2.0',
    url='https://github.com/imagine-engine/imagine',
    zip_safe=False,
    description=(
        'imagine is a simple, but powerful general-purpose animation tool'
    ),
    packages=find_namespace_packages(include=['imagine.*']),
    rust_extensions=[
        RustExtension(
            'imagine',
            binding=Binding.PyO3,
            # args= (
            #     ['--preview=True'] if any(arg=='--preview' for arg in sys.argv)
            #     else ['--preview=False']
            # ),
            debug=False
        )
    ],
    classifiers=[
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: Apache Software License',
        'Programming Language :: Rust',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',
        'Programming Language :: Python :: 3.9',
        'Programming Language :: Python :: 3.10',
        'Programming Language :: Python :: 3.11',
        'Topic :: Multimedia :: Video',
        'Topic :: Multimedia :: Graphics :: 3D Rendering',
        'Topic :: Scientific/Engineering :: Physics',
        'Topic :: Scientific/Engineering :: Mathematics',
        'Topic :: Scientific/Engineering :: Visualization',
        'Topic :: Software Development :: Libraries',
        'Topic :: Software Development :: Libraries :: Python Modules'
    ]
)