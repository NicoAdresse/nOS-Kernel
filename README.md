# nOS-Kernel — A Tiny x86_64 Hobby Kernel

nOS is a small, experimental x86_64 operating system written in Rust.

# How Does It Work

It boots via a custom 64-bit bootloader (boot64.asm), enters long mode, and hands control to a minimal Rust kernel with a handcrafted VGA text subsystem.

This project is a playground for systems programming, OS design, and low-level Rust — built from scratch, without a `std`, without an existing OS framework, and without external runtime depndencies.

# Features

- Custom 64-bit bootloader (NASM)
- Rust kernel with `#![no_std]`
    - cursor tracking
    - auto-wrap
    - auto-scroll
    - color styles
    - named color themes
- Boot banner with CP437-safe box drawing
- Color-coded logging helpers
    - print_ok
    - print_normal
    - print_err
    - print_info
    - print_warn
    - println_ok
    - println_normal
    - println_err
    - println_info
    - println_warn
- Clean modular structure

# Building

You need:
- Rust nightly
- `rust-src` component
- `nasm`
- `qemu-system-x86_64`

You can find the build script in initialization.txt, copy it and paste it into your terminal while you are in your appropriate directory.

# Project Structure

```

src/
 ├── main.rs          # Kernel entry
 ├── scrolling.rs     # VGA scrolling logic
 └── vga/
      ├── mod.rs
      ├── color.rs
      ├── style.rs
      ├── buffer.rs
      ├── cursor.rs
      └── writer.rs
boot64.asm            # 64-bit bootloader
kernel.ld             # Linker script

```

# Contributing

This is a personal learning project, but PRs, issues, and discussions are welcome. If you're experimenting with OS development or Rust bare-metal, feel free to fork and play.

# How To Customize

Go to `/src/main.rs` and look for the `_start` function.

Then replace the `vga::print_boot_banner();` with any of the VGA writer modules.

Example:

```
vga::print_ok("Everything is fine.");
vga::print_warn("Something looks suspicious...");
vga::print_err("Something exploded!?");
vga::print_normal("Welcome to nOS!");

```

You can write directly to the VGA buffer using:

```
vga::println("Hello,", vga::Style::new(vga::Color::LightCyan, vga::Color::Black));
vga::print(" World!", vga::Style::new(vga::Color::Green, vga::Color::Black));
```

Or used named styles:

`vga::println_named("Hello from nOS!", "white_black");`

### Hack on the VGA Driver

The VGA subsystem is fully modular:
- `buffer.rs` — raw memory writes
- `cursor.rs` — cursor movement
- `writer.rs` — printing logic
- `style.rs` — color/style definitions

Try changing:
- The cursor behavior
- the scroll speed
- the default colors
- the screen clear behavior

### How to change the VGA banner

Go into `/src/vga/writer.rs` and scroll down to the function `print_boot_banner` and replace it with anything you like.

# License

MIT — Check out the file: `LICENSE`
