
# Rendering Pipeline

The rendering pipeline should be as self-contained as possible, so
instead of giving everything access to rendering, only a system or two
should actually be rendering anything.

Potential options for rendering are just raw OpenGL bindings, either
through Gluim or through Glow (with added benefit of WebGL potential),
or through a game engine like Piston. Other options typically require
Vulkan, which makes them too bleeding edge for our current needs.

## Glow

An interesting case of multi-platform OpenGL is Glow, which is capable
of supporting browser targets. The issue with multi-platform
targetting as always is the same as always, you'll endup with
duplicate code, whether you like it or not.
