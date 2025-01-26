# Keyboard macros for [MISide](https://store.steampowered.com/app/2527500/MiSide/) speedrun.

## Features
- Set  ***TriggerKey*** (mouse or keyboard).
- Adjust duration of sleep time between 1st and 2nd *press-release* `ESC` action via `FPS` settings.

## Installation
- Go to [releases](https://github.com/dehwyy/MISideSpeedrunMacros/releases).
- Download ***.exe*** file from assets.
- Run ***.exe*** file.

### About FPS:
- The more ***fps***  you have in game the faster you can theoretically press `ESC` twice.
- A lot depends on how `MISide` handles (keyboard) events. Would it ignore second press-release of ESC if not enough time passed after 1st action?
- Common behavior may be **batching** same (keyboard) events (2x press-release `ESC` in our case). So, to avoid it, we should wait ***1 frame***, which can be calculated `1 / FPS` = `1 frame time in seconds`.
- Thus, `FPS` setting is just your ingame ***fps***.

I would do some investigation on this behavior later and edit description.
