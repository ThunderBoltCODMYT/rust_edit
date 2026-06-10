# rust_edit

Just a lightweight toy terminal editor I wrote in Rust to mess around with terminal logic and see if I could make one that works.

> [!NOTE]
> Licensed under MIT

### What it does:
- **Basic typing & movement:** You can type and move around using the arrow keys.
- **Lazy saving:** It doesn't autosave constantly. It only writes to the file when you hit `Ctrl + S`.
- **Filename check:** Before saving, it checks if the filename is valid for the OS you're running on (desktop or embedded) using conditional compilation, so it doesn't crash or break things.
  - reason: respect users sanity
- **Easy quit:** Just press `q` to close it.

### A small tweak for performance:
- **Dirty lines tracking:** To stop the screen from flickering, it doesn't redraw the whole screen every time you press a key. It tracks which lines actually changed and only updates those rows.

# How to use it:

You can build it from source by executing this (you need to have rust and cargo installed tho) :
```
git clone https://github.com/thunderboltcodmyt/rust_edit
cd rust_edit/
cargo build
```
