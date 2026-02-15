# Orbital Display Stack

Orbital is the windowing system and compositor for Redox OS. Unlike traditional monolithic display servers like X11, Orbital is built with modern principles of security, performance, and simplicity.

## Architecture

Orbital consists of several key layers:

- **Orbital Server**: The compositor itself. It manages window placement, Z-ordering, and handles input events.
- **Orbclient**: The client-side library used by applications to talk to the Orbital server.
- **Display Schemes**: Orbital exposes display resources through the `/scheme/display` and `/scheme/orbital` schemes.

## Features

- **Double Buffering**: All window drawing is double-buffered to prevent flickering and ensure smooth animations.
- **Microkernel Isolation**: If the compositor crashes, individual applications remain stable in memory (though they may need to reconnect to a new display instance).
- **Universal Input**: Orbital leverages the system's `inputd` to handle everything from PS/2 mice to multi-touch USB devices.

## Graphics Libraries

Orbital supports several high-level Rust graphics libraries, allowing modern applications to run natively:

- **Winit**: Cross-platform window creation and event handling.
- **Softbuffer**: Cross-platform software rendering.
- **Slint / Iced / Egui**: Modern, declarative UI frameworks.

## Performance

By using specialized shared memory regions for window buffers, Orbital achieves near-zero copy rendering for 2D UI elements.
