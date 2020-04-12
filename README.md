# `enact`

`enact` will detect the proper resolution of your secondary monitor (if any) and automatically set it up as soon as you plug it in (or out).

It uses `xrandr` under the hood and works great with window managers like i3, bspwm, and others.

Use cases:
- a laptop and an abritrary secondary monitor (e.g. at work, home, etc.)
- a desktop with two monitors

## Usage

Test it out then place this in your `.xinitrc`.

```sh
# Set up second monitor above laptop and exit
enact --pos top

# Do the same, but watch for changes and allow hotplugging
enact --pos top --watch
```
