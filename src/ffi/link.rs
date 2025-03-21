// Copyright 2014 The GLFW-RS Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(feature = "glfw-sys")]
#[link(name = "glfw3", kind = "static")]
extern "C" {}

#[cfg(not(feature = "glfw-sys"))]
// leaving off `kind = static` allows for the specification of a dynamic library if desired
#[cfg(target_family = "unix")]
#[link(name = "glfw3", kind = "static")]
extern "C" {}

#[cfg(not(feature = "glfw-sys"))]
#[cfg(target_family = "windows")]
#[link(name = "glfw3", kind = "static")]
extern "C" {}

#[cfg(target_os = "windows")]
#[link(name = "opengl32")]
#[link(name = "gdi32")]
#[link(name = "user32")]
#[link(name = "shell32")]
extern "C" {}

#[cfg(all(
    any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly"),
    feature = "wayland"
))]
#[link(name = "wayland-client")]
extern "C" {}

#[cfg(all(
    any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly"),
    not(feature = "wayland")
))]
#[link(name = "X11")]
extern "C" {}

#[cfg(target_os = "macos")]
#[link(name = "Cocoa", kind = "framework")]
#[link(name = "OpenGL", kind = "framework")]
#[link(name = "IOKit", kind = "framework")]
#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "QuartzCore", kind = "framework")]
extern "C" {}
