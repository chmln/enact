<h1 align=center> <img src="https://user-images.githubusercontent.com/11352152/79083479-91ec3280-7cfc-11ea-9f81-045acc4f8ec0.png" width=64 align=top /><br/>enact</h1>


`enact` will detect the proper resolution of your secondary monitor (if any) and automatically set it up as soon as you plug it in (or out).

It uses `xrandr` under the hood and works great with window managers like i3, bspwm, and others.

Use cases:
- a laptop and an arbitrary secondary monitor (e.g. at work, home, etc.)
- a desktop with two monitors

## Install

Download the binary from [releases](https://github.com/chmln/enact/releases) or install via cargo: `cargo install --git https://github.com/chmln/enact`

## Usage

Test it out then place this in your `.xinitrc`.

```sh
# Set up second monitor above laptop
enact --pos top
```

Or to do the same, but also watch for changes and allow hotplugging

```sh
enact --pos top --watch &
```

You can also select which monitor will be the new primary one

```sh
enact --pos top --new-primary 1
```

## Comparison With Similar Tools

Pros:
- monitor hotplugging that actually works (never got this to work with autorandr or any other tool)
- no need to setup any "profiles" or configuration, it just works
- Single compiled binary, no dependencies on python or anything else apart from `xrandr`

Drawbacks:
- Supports up to two displays max (at least currently)

## Icon Attribution

[“Monitor”](https://www.iconfinder.com/icons/4064140/computer_hardware_monitor_screen_technology_icon) by [icon lauk](https://www.iconfinder.com/andhikairfani), licensed under CC BY 3.0.
