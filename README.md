# 🖼️ Steam Animated Avatars

This Garry's Mod addon is a drop-in, backwards-compatible replacement for AvatarImage that enables support for Steam's animated avatars.

https://github.com/user-attachments/assets/c206b442-a5e2-4737-8771-b7a0f27dd110

## How does it work?

This addon leverages WebAssembly (WASM) to run a GIF-to-VTF conversion library directly within Garry's Mod via a [DHTML](https://wiki.facepunch.com/gmod/DHTML) panel.

When an animated avatar is requested, the addon fetches the GIF data from Steam, processes it using the WASM module, and converts it into a VTF format that Garry's Mod can render.

The VTF is written to the data/ folder, and the addon then displays it in-game.

## Installation

You can install this addon via the [Workshop]().

If added to a server collection, clients will automatically download it when they join the server.

## Global `AvatarImage` Override

This addon automatically overrides the global `AvatarImage` panel with the animated version.

As they're 1:1 compatible, no code changes are necessary to benefit from animated avatars in your existing projects, and this should not clash with any existing addons.

Everything should just work!

This behaviour can be disabled on the client by running:

```lua
cl_animated_avatars 0
```

in console.

## Lua API

This addon registers the `AnimatedAvatarImage` VGUI panel.

It's completely 1:1 with [`AvatarImage`](https://wiki.facepunch.com/gmod/AvatarImage) except it automatically downloads and renders animated Steam avatars.

Simply change your:

```lua
local avatar = vgui.Create("AvatarImage")
```

to:

```lua
local avatar = vgui.Create("AnimatedAvatarImage")
```

and enjoy animated avatars!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
