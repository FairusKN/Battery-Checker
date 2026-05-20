# Battery Checker 🔋

A lightweight custom battery checker for Linux based on rust.

Built with:
- Rust 🦀
- `tokio`
- `tokio-udev`
- Linux `sysfs`
- `notify-send`
- `swayosd-client`

---

# Features

- Event-driven (no aggressive polling)
- Lightweight
- Simple architecture
- Wayland-friendly
- Works well with Hyprland/Sway setups
- Uses native Linux power subsystem

---

# Requirements

## System Requirements

Linux system with:

- `udev`
- `/sys/class/power_supply`
- Battery device exposed as:
    - `/sys/class/power_supply/BAT0`

Check with:

```bash
ls /sys/class/power_supply

