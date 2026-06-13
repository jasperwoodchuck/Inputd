# Inputd

A blazingly fast and lightweight input remapper for Linux, written in Rust.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black)
![Wayland](https://img.shields.io/badge/Wayland-FFBC00?style=flat-square&logo=wayland&logoColor=black)
![Git](https://img.shields.io/badge/Git-F05032?style=flat-square&logo=git&logoColor=white)
![GitHub](https://img.shields.io/badge/GitHub-181717?style=flat-square&logo=github&logoColor=white)

## Roadmap

- [x] Implement a custom DSL parser
- [x] Input remapping
	- Keyboard
	- Mouse
	- Combination
- [x] Key disabling support
- [x] Emergency exit shortcut
- [ ] Config hot reloading
- [ ] Macro support
- [ ] Daemon mode
- [ ] More features...
- [ ] First stable release
- [ ] Package for major Linux distros
	- [ ] Arch User Repository
	- [ ] Fedora (.rpm)
	- [ ] Debian (.deb)

## Configuration

Inputd uses a custom DSL to define keyboard and mouse remappings.

### Example Configuration

The configuration file is located at `~/.config/inputd/config.binds`.

```binds
# This comment line will be ignored

Keyboard="/by-path/platform-i8042-serio-0-event-kbd"
Mouse="/by-id/usb-Logitech_USB_Receiver-if02-event-mouse"

MOUSE5 + WHEEL_UP   :: WHEEL_RIGHT
MOUSE5 + WHEEL_DOWN :: WHEEL_LEFT

MOUSE5 :: Disable

CAPSLOCK :: ESC

CTRL + H :: LEFT
CTRL + J :: DOWN
CTRL + K :: UP
CTRL + L :: RIGHT
```

### Emergency Exit Shortcut

> [!WARNING]
> Inputd provides a hardcoded emergency exit shortcut to recover from broken
> configurations or device grabs that render normal input unavailable.
>
> Press `Ctrl + Alt + Shift + Delete` simultaneously, in any order, to
> immediately terminate Inputd.
>
> This shortcut is always available and cannot be remapped or disabled through
> the configuration.

### Clear Error Messages

If the configuration contains an error:

```
invalid input
---> 9:11
  |
9 | MOUSE5 :: Disabl
  |           ^^^^^^
```

Inputd reports syntax and configuration errors with precise line and column information, making configuration issues easy to identify and resolve.

To see all available input tokens, refer to the `str_to_input()` function in
[`src/input/mapping.rs`](src/input/mapping.rs).

> [!NOTE]
> A dedicated `SUPPORTED_KEYS.md` reference will be added in a future release.

# License

This project is licensed under the [GPL-3.0 License](LICENSE).
