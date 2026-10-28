import os
import sys

from SCons.Script import Environment

env = Environment()

target_path = "bin"
target_name = "orthogonal_router"

env.Append(CPPPATH=["."])
env.Append(CXXFLAGS=["/std:c++20", "/O2"])

sources = env.Glob("*.c") + env.Glob("*.cpp")

library = env.SharedLibrary(
    target=os.path.join(target_path, target_name),
    source=sources,
)

env.Default(library)
